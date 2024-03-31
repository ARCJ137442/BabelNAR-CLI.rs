//! 定义一个抽象特征，用于「命令行虚拟机」的命令参数/执行环境 自动生成

use std::process::Command;

/// 命令生成器
/// * 🎯主管[`Command`]对象的**模板化生成**
///   * 📄OpenNARS使用`java`命令生成器，允许在指定转译器的同时自定义Java启动参数
pub trait CommandGenerator {
    /// 通过自身内部参数，生成指令参数
    fn generate_command(&self) -> Command;
}
