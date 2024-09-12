#![allow(unused_variables)]
#![allow(dead_code)]

nar_dev_utils::macro_once! {
    /// 实用宏/简化字符串常量
    macro str_const($(
        $(#[$m:meta])*
        $name:ident = $value:literal $(;)?
    )*) {
        $(
            $(#[$m])*
            pub const $name: &str = $value;
        )*
    }

    /// 用于「启动参数解析」的测试环境
    ARG_PARSE_TEST =
        "./src/tests/cli/config/_arg_parse_test.opennars.hjson"

    /// OpenNARS
    OPENNARS = "./src/tests/cli/config/cin_opennars.hjson"
    /// OpenNARS
    OPENNARS_158 = "./src/tests/cli/config/cin_opennars_158.hjson"
    /// ONA
    ONA = "./src/tests/cli/config/cin_ona.hjson"
    /// PyNARS
    PYNARS = "./src/tests/cli/config/cin_pynars.hjson"
    /// CXinJS
    CXIN_JS = "./src/tests/cli/config/cin_cxin_js.hjson"
    /// 原生IL-1
    NATIVE_IL_1 = "./src/tests/cli/config/cin_native_il_1.hjson"
    /// NARust-158
    NARUST_158 = "./src/tests/cli/config/cin_narust_158.hjson"

    /// 预引入/NAL测试环境
    PRELUDE_TEST = "./src/tests/cli/config/prelude_test.hjson"
    /// NAL/简单演绎
    NAL_SIMPLE_DEDUCTION = "./src/tests/cli/config/nal_simple_deduction.hjson"
    /// NAL/高阶演绎
    NAL_HIGHER_DEDUCTION = "./src/tests/cli/config/nal_higher_deduction.hjson"
    /// NAL/自变量消除
    NAL_I_VAR_ELIMINATION = "./src/tests/cli/config/nal_i_var_elimination.hjson"
    /// NAL/时间归纳
    NAL_TEMPORAL_INDUCTION = "./src/tests/cli/config/nal_temporal_induction.hjson"
    /// NAL/操作
    NAL_OPERATION = "./src/tests/cli/config/nal_operation.hjson"
    /// NAL/简单操作
    NAL_SIMPLE_OPERATION = "./src/tests/cli/config/nal_simple_operation.hjson"
    /// NAL/真值通配
    NAL_TRUTH_WILDCARD = "./src/tests/cli/config/nal_truth_wildcard.hjson"

    /// Websocket
    WEBSOCKET = "./src/tests/cli/config/websocket.hjson"
    /// Matriangle服务器
    MATRIANGLE_SERVER = "./src/tests/cli/config/matriangle_server.hjson"
}
