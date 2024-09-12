//! 原BabelNAR.rs `src/bin/babelnar_cli/*.rs`
//! * 🚩【2024-09-12 17:41:35】现在统一放置在`src/cli`下

nar_dev_utils::mods! {
    // 启动参数
    pub use vm_config;
    // 命令行解析
    pub use arg_parse;
    // 配置（自动）搜索
    pub use config_search;
    // 从配置启动
    pub use config_launcher;
    // 运行时交互、管理
    pub use runtime_manage;
    // Websocket服务端
    pub use websocket_server;
}
