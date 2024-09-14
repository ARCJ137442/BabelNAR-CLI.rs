# BabelNAR-CLI.rs

|**简体中文** | [English](README.en.md)|
|:-:|:-:|

    🏗️项目的**英文文档**尚在筹建，有意者欢迎提交PR

<!-- 徽章安排参考：https://daily.dev/blog/readme-badges-github-best-practices#organizing-badges-in-your-readme -->

![License](https://img.shields.io/crates/l/babel_nar_cli?style=for-the-badge&color=ff7043)
![Code Size](https://img.shields.io/github/languages/code-size/ARCJ137442/BabelNAR-CLI.rs?style=for-the-badge&color=ff7043)
![Lines of Code](https://www.aschey.tech/tokei/github.com/ARCJ137442/BabelNAR-CLI.rs?style=for-the-badge&color=ff7043)
[![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=ff7043)](https://www.rust-lang.org)

<!-- 面向用户 -->

Cargo状态：

[![crates.io](https://img.shields.io/crates/v/babel_nar_cli?style=for-the-badge)](https://crates.io/crates/babel_nar_cli)
[![docs.rs](https://img.shields.io/docsrs/babel_nar_cli?style=for-the-badge)](https://docs.rs/babel_nar_cli)
![Crate Size](https://img.shields.io/crates/size/babel_nar_cli?style=for-the-badge)

![Recent Downloads](https://img.shields.io/crates/dr/babel_nar_cli?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/babel_nar_cli?style=for-the-badge)
![Crates.io Dependents](https://img.shields.io/crates/dependents/babel_nar_cli?style=for-the-badge)

<!-- 面向开发者 -->

开发状态：

[![CI status](https://img.shields.io/github/actions/workflow/status/ARCJ137442/BabelNAR-CLI.rs/ci.yml?style=for-the-badge)](https://github.com/ARCJ137442/BabelNAR-CLI.rs/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?style=for-the-badge)](https://conventionalcommits.org)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/ARCJ137442/BabelNAR-CLI.rs/latest?style=for-the-badge)

![Created At](https://img.shields.io/github/created-at/ARCJ137442/BabelNAR-CLI.rs?style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/ARCJ137442/BabelNAR-CLI.rs?style=for-the-badge)

## 简介

[**BabelNAR.rs**](https://github.com/ARCJ137442/BabelNAR.rs)的命令行接口(CLI)

- ✨从配置中快速启动、测试各类NARS实现
  - 🎯一站式NARS**启动器**
  - 🎯NARS**交叉测试**工具

<!-- ## 安装 -->

<!-- * 📌【2024-04-10 10:19:40】有关具体环节，在crates.io中已经完善 -->

## 使用

- 依照使用 JSON/HJSON 配置文件 (`.json`/`.hjson`)
  - 格式可参考
    - `config_public` 中的配置文件
    - 项目测试代码
- Rust调用侧：可参考项目测试代码

🏗️TODO（接受贡献）

## CLI测试：各CIN完成度评估

🕒最后更新时间：【2024-09-13 01:34:32】

|  | 简单演绎 | 高阶演绎 | 自变量消除 | 时间归纳 | 简单操作 | 时序操作 |
| :--- | :--: | :--: | :--: | :--: | :--: | :--: |
| 原理 | 继承关系的传递性 | 蕴含关系的蕴含保真 | 代入消元 | 前后事件的联系 | 直接要求「做某事」 | 在「发生某事，做某事，目标达成」中学会「若发生某事，就做某事」 |
| 对应NAL内容 | NAL-1 | NAL-5 | NAL-5 + NAL-6 | NAL-7 | NAL-8 | NAL-7 + NAL-8 |
| 语句输入 | `<A --> B>.` + `<B --> C>.` | `<<A --> B> ==> <C --> D>>.` + `<A --> B>.` | `<<A --> $1> ==> <$1 --> C>>.` + `<A --> B>.` | `<A --> B>. :\|:` + `<C --> D>. :\|:` | `<(*, ...) --> ^left>! :\|:` | `A. :\|:` + `<(*, {SELF}) --> ^left>. :\|:` + `G. :\|:` + `<(&/, A, <(*, ...) --> ^left>) ==> G>?` + `G! :\|:` |
| 预期输出 | `<A --> C>.` | `<C --> D>.` | `<B --> C>.` | `<<A --> B> =/> <C --> D>>.` | EXE `<(*, ...) --> ^left> :\|:` | EXE `<(&/, A, <(*, ...) --> ^left>) ==> G>` |
| OpenNARS(3.0.4) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| OpenNARS(1.5.8) | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| ONA | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| PyNARS | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| CXinNARS | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| NARust-158 | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |

## 参考

- [BabelNAR.rs](https://github.com/ARCJ137442/BabelNAR.rs)
