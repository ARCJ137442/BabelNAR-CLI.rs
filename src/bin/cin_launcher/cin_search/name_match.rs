//! 封装「名称匹配」逻辑
//! * 🎯用于「语义化」「模糊化」的字符串匹配
//!   * ✨无视大小写匹配
//!     * 📄"opennars"匹配"OpenNARS"
//!   * ✨「含于」与「包含」匹配
//!     * 📄"opennars"匹配"OpenNARS 3.0.4"（含于）与"nars"（包含）
//!   * ✨返回一个「匹配度」的数值
//!     * `0`统一表示「未匹配」
//!     * 剩余值可用于排序

use nar_dev_utils::{first, if_return};

/// 名称匹配
/// * 🎯用于「语义化」「模糊化」的字符串匹配
///   * ✨无视大小写匹配
///     * 📄"opennars"匹配"OpenNARS"
///   * ✨「含于」与「包含」匹配
///     * 📄"opennars"匹配"OpenNARS 3.0.4"（含于）与"nars"（包含）
/// * ⚙️返回一个「匹配度」的数值
///   * `0`统一表示「未匹配」
///   * 剩余值可用于排序
pub fn name_match(name: &str, target: &str) -> usize {
    // 完全相等⇒最高级
    if_return! {
        // 完全相等⇒高
        name == target => 6
        // 包含于⇒中
        target.contains(name) => 4
        // 包含⇒低
        name.contains(target) => 2
    }

    // 忽略大小写的情况 | 忽略大小写，降一个匹配度
    let name = name.to_lowercase();
    let target = target.to_lowercase();

    first! {
        // 完全相等⇒高
        name == target => 5,
        // 包含于⇒中
        target.contains(&name) => 3,
        // 包含⇒低
        name.contains(&target) => 1,
        // 否则⇒不匹配
        _ => 0,
    }
}

/// 名称匹配/仅「含于」
/// * 🚩与[`name_match`]类似，但仅「含于」而不适配「包含」
/// * 🎯用于「长串名称作为内部关键词」的匹配
pub fn name_match_only_contains(name: &str, target: &str) -> usize {
    // 完全相等⇒最高级
    if_return! {
        // 完全相等⇒高
        name == target => 4
        // 含于⇒低
        target.contains(name) => 2
    }

    // 忽略大小写的情况 | 忽略大小写，降一个匹配度
    let name = name.to_lowercase();
    let target = target.to_lowercase();

    first! {
        // 完全相等⇒高
        name == target => 3,
        // 含于⇒低
        target.contains(&name) => 1,
        // 否则⇒不匹配
        _ => 0,
    }
}

/// 判断「是否匹配」，不管「匹配度」多少
/// * 🚩直接复用逻辑，以牺牲一定性能为代价
pub fn is_name_match(name: &str, target: &str) -> bool {
    name_match(name, target) > 0
}
