//! 原BabelNAR.rs `src/bin/babelnar_cli/*.rs`
//! * 🚩【2024-09-12 17:41:35】现在统一放置在`src/cli`下

use crate::{eprintln_cli, println_cli};
use anyhow::Result;
use clap::Parser;
use std::{io::Result as IoResult, path::PathBuf, thread::sleep, time::Duration};

// 启动参数
mod vm_config;
pub use vm_config::*;

// 命令行解析
mod arg_parse;
pub use arg_parse::*;

// 配置（自动）搜索
mod config_search;
pub use config_search::*;

// 从配置启动
mod config_launcher;
pub use config_launcher::*;

// 运行时交互、管理
mod runtime_manage;
pub use runtime_manage::*;

// Websocket服务端
mod websocket_server;
pub use websocket_server::*;

/// 以特定参数开始命令行主程序
/// * 🚩此处只应该有自[`env`]传入的参数
/// * 🚩【2024-04-01 14:25:38】暂时用不到「当前工作路径」
pub fn main_args(cwd: IoResult<PathBuf>, args: impl Iterator<Item = String>) -> Result<()> {
    // 解包当前工作目录
    let cwd = cwd
        .inspect_err(|e| println_cli!([Warn] "无法获取当前工作目录：{e}"))
        .ok();

    // （Windows下）启用终端颜色
    let _ = colored::control::set_virtual_terminal(true)
        .inspect_err(|_| eprintln_cli!([Error] "无法启动终端彩色显示。。"));

    // 解析命令行参数
    let args = CliArgs::parse_from(args);

    // 读取配置 | with 默认配置文件
    let mut config = load_config(&args);

    // 是否向用户展示「详细信息」 | 用于等待、提示等
    let user_verbose = config.user_input.is_none() || config.user_input.unwrap();

    // 用户填充配置项 | 需要用户输入、工作路径（🎯自动搜索）
    polyfill_config_from_user(&mut config, cwd);

    // 清屏，预备启动
    if user_verbose {
        println_cli!([Info] "配置加载完毕！程序将在1s后启动。。。");
        sleep(Duration::from_secs(1));
    }
    let _ = clearscreen::clear().inspect_err(|e| eprintln_cli!([Warn] "清屏失败：{e}"));

    // 从配置项启动 | 复制一个新配置，不会附带任何非基础类型开销
    let (runtime, config) = match launch_by_config(config.clone()) {
        // 启动成功⇒返回
        Ok((r, c)) => (r, c),
        // 启动失败⇒打印错误信息，等待并退出
        Err(e) => {
            println_cli!([Error] "NARS运行时启动错误：{e}");
            // 空配置/启用用户输入⇒延时提示
            if user_verbose {
                println_cli!([Info] "程序将在 3 秒后自动退出。。。");
                sleep(Duration::from_secs(3));
            }
            return Err(e);
        }
    };

    // 运行时交互、管理
    let manager = RuntimeManager::new(runtime, config.clone());
    let result = loop_manage(manager, &config);

    // 启用用户输入时延时提示
    if config.user_input {
        println_cli!([Info] "程序将在 5 秒后自动退出。。。");
        sleep(Duration::from_secs(3));
    }

    // 返回结果
    result
}
