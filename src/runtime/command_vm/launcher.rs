//! 命令行虚拟机（构建者）

use super::{InputTranslator, OutputTranslator};
use crate::process_io::IoProcess;
use navm::{cmd::Cmd, output::Output};
use std::ffi::OsStr;

/// 命令行虚拟机（构建者）
/// * 🎯配置化构造[`CommandVmRuntime`]
///   * 封装内部「输入输出进程」的「输出侦听器」逻辑
/// * 🚩有关「启动」的流程，放在「虚拟机运行时」[`super::runtime`]中
pub struct CommandVm {
    /// 内部存储的「输入输出进程」
    pub(super) io_process: IoProcess,

    /// [`Cmd`]→进程输入 转译器
    pub(super) input_translator: Option<Box<InputTranslator>>,

    /// 进程输出→[`Output`]转译器
    pub(super) output_translator: Option<Box<OutputTranslator>>,
}

impl CommandVm {
    /// 构造函数
    pub fn new(program_path: impl AsRef<OsStr>) -> Self {
        Self {
            // 指令
            io_process: IoProcess::new(program_path),
            // 其它暂时置空
            input_translator: None,
            output_translator: None,
        }
    }

    /// 配置/输入转换器
    /// * 💭何时Rust能给特征起别名。。
    pub fn input_translator(
        mut self,
        translator: impl Fn(Cmd) -> Result<String, String> + Send + Sync + 'static,
    ) -> Self {
        self.input_translator = Some(Box::new(translator));
        self
    }

    /// 配置/输出转换器
    pub fn output_translator(
        mut self,
        translator: impl Fn(String) -> Result<Output, String> + Send + Sync + 'static,
    ) -> Self {
        self.output_translator = Some(Box::new(translator));
        self
    }
}
