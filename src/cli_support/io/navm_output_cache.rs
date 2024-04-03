//! NAVM输出缓存
//! * 🎯一站式存储、展示与管理NAVM的输出
//! * 🎯可被其它二进制库所复用

use crate::{cli_support::error_handling_boost::error_anyhow, test_tools::VmOutputCache};
use anyhow::Result;
use nar_dev_utils::ResultBoost;
use navm::output::Output;
use std::{
    ops::ControlFlow,
    sync::{Arc, Mutex, MutexGuard},
};

use super::output_print::OutputType;

/// 线程间可变引用计数的别名
pub type ArcMutex<T> = Arc<Mutex<T>>;

/// 输出缓存
/// * 🎯统一「加入输出⇒打印输出」的逻辑
/// * 🚩仅封装一个[`Vec`]，而不对其附加任何[`Arc`]、[`Mutex`]的限定
///   * ❌【2024-04-03 01:43:13】[`Arc`]必须留给[`RuntimeManager`]：需要对其中键的值进行引用
#[derive(Debug)]
pub struct OutputCache {
    /// 内部封装的输出数组
    /// * 🚩【2024-04-03 01:43:41】不附带任何包装类型，仅包装其自身
    inner: Vec<Output>,
}

/// 功能实现
impl OutputCache {
    /// 构造函数
    pub fn new(inner: Vec<Output>) -> Self {
        Self { inner }
    }

    /// 不可变借用内部
    pub fn borrow_inner(&self) -> &Vec<Output> {
        &self.inner
    }

    /// 可变借用内部
    pub fn borrow_inner_mut(&mut self) -> &mut Vec<Output> {
        &mut self.inner
    }

    /// 默认[`Arc`]<[`Mutex`]>
    pub fn default_arc_mutex() -> ArcMutex<Self> {
        Arc::new(Mutex::new(Self::default()))
    }

    /// 从[`Arc`]<[`Mutex`]>中解锁
    pub fn unlock_arc_mutex(arc_mutex: &mut ArcMutex<Self>) -> Result<MutexGuard<'_, Self>> {
        arc_mutex.lock().transform_err(error_anyhow)
    }

    /// 静默存入输出
    /// * 🎯内部可用的「静默存入输出」逻辑
    ///   * 🚩【2024-04-03 01:07:55】不打算封装了
    pub fn put_silent(&mut self, output: Output) -> Result<()> {
        // 加入输出
        self.inner.push(output);
        Ok(())
    }
}

/// 默认构造：空数组
impl Default for OutputCache {
    fn default() -> Self {
        Self::new(vec![])
    }
}

/// 实现「输出缓存」
/// * 🚩【2024-04-03 14:33:50】不再涉及任何[`Arc`]或[`Mutex`]
impl VmOutputCache for OutputCache {
    /// 存入输出
    /// * 🎯统一的「打印输出」逻辑
    ///   * 🚩【2024-04-03 01:07:55】不打算封装了
    fn put(&mut self, output: Output) -> Result<()> {
        // 打印输出
        // * 🚩现在内置入「命令行支持」，不再能直接使用`println_cli`
        OutputType::print_from_navm_output(&output);

        // 静默加入输出
        self.put_silent(output)
    }

    /// 遍历输出
    /// * 🚩不是返回迭代器，而是用闭包开始计算
    fn for_each<T>(&self, f: impl Fn(&Output) -> ControlFlow<T>) -> Result<Option<T>> {
        // 遍历
        for output in self.inner.iter() {
            // 基于控制流的运行
            match f(output) {
                ControlFlow::Break(value) => return Ok(Some(value)),
                ControlFlow::Continue(()) => {}
            }
        }

        // 返回
        Ok(None)
    }
}
