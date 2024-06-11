//! 与NAVM虚拟机的交互逻辑

use super::{NALInput, OutputExpectation, OutputExpectationError};
use crate::cli_support::{error_handling_boost::error_anyhow, io::output_print::OutputType};
use anyhow::Result;
use nar_dev_utils::{if_return, ResultBoost};
use navm::{cmd::Cmd, output::Output, vm::VmRuntime};
use std::{ops::ControlFlow, path::Path};

// Narsese预期
mod narsese_expectation;
pub use narsese_expectation::*;

// 词项判等
mod term_equal;

/// 实现/预期匹配功能
impl OutputExpectation {
    /// 判断一个「NAVM输出」是否与自身相符合
    pub fn matches(&self, output: &Output) -> bool {
        // 输出类型
        if let Some(expected) = &self.output_type {
            if_return! { expected != output.type_name() => false }
        }

        // Narsese
        match (&self.narsese, output.get_narsese()) {
            // 预期有，输出无⇒直接pass
            (Some(..), None) => return false,
            // 预期输出都有⇒判断Narsese是否相同
            (Some(expected), Some(out)) => {
                if_return! { !is_expected_narsese_lexical(expected, out) => false }
            }
            _ => (),
        }

        // 操作 | 最后返回
        match (&self.operation, output.get_operation()) {
            // 预期无⇒通配
            (None, ..) => true,
            // 预期有，输出无⇒直接pass
            (Some(_), None) => false,
            // 预期有，输出有⇒判断操作是否相同
            (Some(expected), Some(out)) => is_expected_operation(expected, out),
        }
    }
}

/// 输出缓存
/// * 🎯为「使用『推送』功能，而不引入具体数据类型」设置
/// * 📌基础功能：推送输出、遍历输出
pub trait VmOutputCache {
    /// 存入输出
    /// * 🎯统一的「打印输出」逻辑
    fn put(&mut self, output: Output) -> Result<()>;

    /// 遍历输出
    /// * 🚩不是返回迭代器，而是用闭包开始计算
    /// * 📝使用最新的「控制流」数据结构
    ///   * 使用[`None`]代表「一路下来没`break`」
    fn for_each<T>(&self, f: impl FnMut(&Output) -> ControlFlow<T>) -> Result<Option<T>>;
}

/// 向虚拟机置入[`NALInput`]
/// * 🎯除了「输入指令」之外，还附带其它逻辑
/// * 🚩通过「输出缓存」参数，解决「缓存输出」问题
/// * ❓需要迁移「符合预期」的逻辑
pub fn put_nal(
    vm: &mut impl VmRuntime,
    input: NALInput,
    output_cache: &mut impl VmOutputCache,
    // 不能传入「启动配置」，就要传入「是否启用用户输入」状态变量
    enabled_user_input: bool,
    nal_root_path: &Path,
) -> Result<()> {
    match input {
        // 置入NAVM指令
        NALInput::Put(cmd) => vm.input_cmd(cmd),
        // 睡眠
        NALInput::Sleep(duration) => {
            // 睡眠指定时间
            std::thread::sleep(duration);
            // 返回`ok`
            Ok(())
        }
        // 等待一个符合预期的NAVM输出
        NALInput::Await(expectation) => loop {
            let output = match vm.fetch_output() {
                Ok(output) => {
                    // 加入缓存
                    output_cache.put(output.clone())?;
                    // ! ❌【2024-04-03 01:19:06】无法再返回引用：不再能直接操作数组，MutexGuard也不允许返回引用
                    // output_cache.last().unwrap()
                    output
                }
                Err(e) => {
                    println!("尝试拉取输出出错：{e}");
                    continue;
                }
            };
            // 只有匹配了才返回
            if expectation.matches(&output) {
                break Ok(());
            }
        },
        // 检查是否有NAVM输出符合预期
        NALInput::ExpectContains(expectation) => {
            // 先尝试拉取所有输出到「输出缓存」
            while let Some(output) = vm.try_fetch_output()? {
                output_cache.put(output)?;
            }
            // 然后读取并匹配缓存
            let result = output_cache.for_each(|output| match expectation.matches(output) {
                true => ControlFlow::Break(true),
                false => ControlFlow::Continue(()),
            })?;
            match result {
                // 只有匹配到了一个，才返回Ok
                Some(true) => Ok(()),
                // 否则返回Err
                _ => Err(OutputExpectationError::ExpectedNotExists(expectation).into()),
            }
            // for output in output_cache.for_each() {
            //     // 只有匹配了才返回Ok
            //     if expectation.matches(output) {
            //     }
            // }
        }
        // 检查在指定的「最大步数」内，是否有NAVM输出符合预期（弹性步数`0~最大步数`）
        NALInput::ExpectCycle(max_cycles, step_cycles, step_duration, expectation) => {
            let mut cycles = 0;
            while cycles < max_cycles {
                // 推理步进
                vm.input_cmd(Cmd::CYC(step_cycles))?;
                cycles += step_cycles;
                // 等待指定时长
                if let Some(duration) = step_duration {
                    std::thread::sleep(duration);
                }
                // 先尝试拉取所有输出到「输出缓存」
                while let Some(output) = vm.try_fetch_output()? {
                    output_cache.put(output)?;
                }
                // 然后读取并匹配缓存
                let result = output_cache.for_each(|output| match expectation.matches(output) {
                    true => ControlFlow::Break(true),
                    false => ControlFlow::Continue(()),
                })?;
                // 匹配到一个⇒提前返回Ok
                if let Some(true) = result {
                    OutputType::Info.print_line(&format!("expect-cycle({cycles}): {expectation}"));
                    return Ok(());
                }
            }
            // 步进完所有步数，仍未有匹配⇒返回Err
            Err(OutputExpectationError::ExpectedNotExists(expectation).into())
        }
        // 保存（所有）输出
        // * 🚩输出到一个文本文件中
        // * ✨复合JSON「对象数组」格式
        NALInput::SaveOutputs(path_str) => {
            // 先收集所有输出的字符串
            let mut file_str = "[".to_string();
            output_cache.for_each(|output| {
                // 换行制表
                file_str += "\n\t";
                // 统一追加到字符串中
                file_str += &output.to_json_string();
                // 逗号
                file_str.push(',');
                // 继续
                ControlFlow::<()>::Continue(())
            })?;
            // 删去尾后逗号
            file_str.pop();
            // 换行，终止符
            file_str += "\n]";
            // 保存到文件中 | 使用基于`nal_root_path`的相对路径
            let path = nal_root_path.join(path_str.trim());
            std::fs::write(path, file_str)?;
            // 提示 | ❌【2024-04-09 22:22:04】执行「NAL输入」时，应始终静默
            // println_cli!([Info] "已将所有NAVM输出保存到文件{path:?}");
            // 返回
            Ok(())
        }
        // 终止虚拟机
        NALInput::Terminate {
            if_not_user,
            result,
        } => {
            // 检查前提条件 | 仅「非用户输入」&启用了用户输入 ⇒ 放弃终止
            if_return! { if_not_user && enabled_user_input => Ok(()) }

            // 终止虚拟机
            vm.terminate()?;

            // 返回
            result.transform_err(error_anyhow)
        }
    }
}
