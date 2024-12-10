use super::*;
impl Node {
    pub fn new_none_getter() -> Self {
        Self::new(
            "NoneGetter".into(),
            0,
            Box::new(
                |target_version, var_name, _input_names| match target_version {
                    TargetVersion::V0_3 => {
                        "panic!(\"NoneGetter available in RRTK 0.4+\");\n".into()
                    }
                    TargetVersion::V0_4 => {
                        format!("let {} = make_input_getter(NoneGetter::new());\n", var_name)
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!("let {} = static_reference!(NoneGetter::new());\n", var_name)
                    }
                },
            ),
        )
    }
    pub fn new_quotient_stream() -> Self {
        Self::new(
            "QuotientStream".into(),
            2,
            Box::new(
                |target_version, var_name, input_names: Vec<String>| match target_version {
                    TargetVersion::V0_3 => format!(
                        "let {} = make_input_getter!(QuotientStream::new({}, {}), f32, E);\n",
                        var_name, input_names[0], input_names[1]
                    ),
                    TargetVersion::V0_4 => format!(
                        "let {} = make_input_getter(QuotientStream::new({}, {}));\n",
                        var_name, input_names[0], input_names[1]
                    ),
                    TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                        "let {} = static_reference!(QuotientStream::new({}, {}));\n",
                        var_name, input_names[0], input_names[1]
                    ),
                },
            ),
        )
    }
}
