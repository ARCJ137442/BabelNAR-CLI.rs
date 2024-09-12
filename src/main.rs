//! BabelNAR 命令行接口
//! * ✨提供对BabelNAR的命令行支持
//!
//! ## 命令行参数语法
//!
//! ```
//! usage: BabelNAR [OPTIONS] <INPUT>
//! ```

use anyhow::Result;
use babel_nar_cli::cli::*;
use std::env;

/// 主入口
pub fn main() -> Result<()> {
    // 以默认参数启动
    main_args(env::current_dir(), env::args())
}
