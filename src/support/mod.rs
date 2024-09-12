//! 命令行支持
//! * 🕒原BabelNAR.rs `src/cli_support/*.rs`
//! * 🚩【2024-09-12 17:41:35】现在统一放置在`src/cli`下
//!   * 🎯避免「CLI修改功能需要动上级模块代码」的情况
//! * 🎯通用、可选地复用「CIN启动器」等「命令行工具」的内容
//! * 🎯亦可为后续基于UI的应用提供支持

nar_dev_utils::mods! {
    // CIN搜索
    pub cin_search;

    // 输入输出
    pub io;
}

// 错误处理增强
pub mod error_handling_boost;
