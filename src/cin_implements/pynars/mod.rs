//! 「非公理虚拟机」的PyNARS运行时
//! * 🚩只提供「一行启动」的功能封装
//!   * 🎯无需自行配置「输入输出转译器」
//!
//! * ❌【2024-03-25 13:00:14】目前无法在Rust侧解决「杀死子进程后，Python继续输出无关信息」的问题
//!   * 📄主要形式：子进程结束后打印错误堆栈，输出`OSError: [Errno 22] Invalid argument`
//!   * ❗无法被Rust捕获，可能是Python运行时的问题（输出未链接到管道）

// 转译器
util::mod_and_pub_use! {
    // 转译器
    translators
    // 启动器
    launcher
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::test::{_test_pynars, MODULE_PATH_PYNARS, MODULE_ROOT_PYNARS};
    use navm::vm::VmLauncher;

    #[test]
    fn test() {
        // 从别的地方获取Python模块根目录、模块自身路径
        let root_path = MODULE_ROOT_PYNARS;
        let module_path = MODULE_PATH_PYNARS;
        // 一行代码启动PyNARS | `python -m pynars.Console` @ "..\..\PyNARS-dev"
        let vm = PyNARS::new(root_path, module_path).launch();
        // 直接复用之前对PyNARS的测试
        _test_pynars(vm)
    }
}
