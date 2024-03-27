//! 一个一站式启动各CIN的启动器
//! * 🎯方便启动、管理各「作为NAVM运行时的CIN」的聚合终端
//! * 📌用于集成原先「BabelNAR」「BabelNAR_Implements」两个库
//! * ✨自动根据可执行文件、配置文件、用户输入猜测CIN类型（字符串匹配）
//! * ✨自动查找（可能）可用的CIN可执行文件（文件搜索）
//! * ✨自动启动并管理CIN
//! TODO: 完成代码
#![allow(unused)]

use babel_nar::{ona::ONA, opennars::OpenNARS, runtime::CommandVmRuntime};
use navm::{
    cmd::Cmd,
    output::Output,
    vm::{VmLauncher, VmRuntime},
};
use std::{fmt::Debug, io::stdin};

const TEST_PATH_OPENNARS: &str = r"..\..\NARS-executables\opennars-304-T-modified.jar";
const TEST_PATH_ONA: &str = r"..\..\NARS-executables\NAR.exe";

/// 启动NARS
/// * 🚩【2024-03-27 18:55:07】目前就返回一个测试用的运行时
fn get_nars() -> impl VmLauncher<CommandVmRuntime> {
    OpenNARS::new(TEST_PATH_OPENNARS)
    // ONA::new(TEST_PATH_ONA)
}

/// 主函数
/// TODO: 完成代码
fn main() {
    // 不断开始🔥
    loop {
        start();
    }
}

/// 开始
fn start() {
    let nars = get_nars().launch();
    shell(nars);
}

/// 打印错误
fn println_error(e: &impl Debug) {
    println!("{e:?}");
}

/// 交互式命令行
fn shell(mut nars: CommandVmRuntime) {
    let stdin = stdin();
    let mut input = String::new();
    let mut line;
    'main: while stdin.read_line(&mut input).is_ok() {
        // 一行
        line = input.as_str();

        // 非空⇒解析出NAVM指令，作为输入执行
        if !line.trim().is_empty() {
            if let Ok(cmd) = Cmd::parse(line).inspect_err(println_error) {
                let _ = nars.input_cmd(cmd).inspect_err(println_error);
            }
        }

        // 尝试拉取所有NAVM运行时输出
        while let Ok(Some(output)) = nars.try_fetch_output().inspect_err(println_error) {
            println!("{output:?}");
            if let Output::TERMINATED { .. } = output {
                println!("NAVM已终止运行，正在重启。。。");
                nars.terminate();
                break 'main; // ! 这个告诉Rust编译器，循环必将在此结束
            }
        }

        // 清空缓冲区
        input.clear();
    }
}
