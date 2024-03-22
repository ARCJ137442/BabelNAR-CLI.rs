//! 定义有关「命令行虚拟机」的抽象API

use navm::{cmd::Cmd, output::Output};

/// [`Cmd`]→进程输入 转译器
/// * 🚩现在不再使用特征，以便在`Option<Box<InputTranslator>>`中推断类型
///   * 📝若给上边类型传入值`None`，编译器无法自动推导合适的类型
/// * 📌要求线程稳定
///   * 只有转译功能，没有其它涉及外部的操作（纯函数）
pub type InputTranslator = dyn Fn(Cmd) -> String + Send + Sync;

/// 进程输出→[`Output`]转译器
/// * 🚩现在不再使用特征，以便在`Option<Box<OutputTranslator>>`中推断类型
///   * 📝若给上边类型传入值`None`，编译器无法自动推导合适的类型
/// * 📌要求线程稳定
///   * 只有转译功能，没有其它涉及外部的操作（纯函数）
pub type OutputTranslator = dyn Fn(String) -> Output + Send + Sync;
