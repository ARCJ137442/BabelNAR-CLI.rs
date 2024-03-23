//! 定义有关「命令行虚拟机」的抽象API

use navm::{cmd::Cmd, output::Output};

/// [`Cmd`]→进程输入 转译器
/// * 🚩现在不再使用特征，以便在`Option<Box<InputTranslator>>`中推断类型
///   * 📝若给上边类型传入值`None`，编译器无法自动推导合适的类型
/// * 📌要求线程稳定
///   * 只有转译功能，没有其它涉及外部的操作（纯函数）
/// TODO: 在后续的「NSE指令输入」时，需要通过「自动将『空预算任务』作为语句输入」应对「`$$ A.`→`A.`」的情况
/// * ⚠️转译有可能失败：此时返回并上报错误信息
pub type InputTranslator = dyn Fn(Cmd) -> Result<String, String> + Send + Sync;

/// 进程输出→[`Output`]转译器
/// * 🚩现在不再使用特征，以便在`Option<Box<OutputTranslator>>`中推断类型
///   * 📝若给上边类型传入值`None`，编译器无法自动推导合适的类型
/// * 📌要求线程稳定
///   * 只有转译功能，没有其它涉及外部的操作（纯函数）
pub type OutputTranslator = dyn Fn(String) -> Result<Output, String> + Send + Sync;
