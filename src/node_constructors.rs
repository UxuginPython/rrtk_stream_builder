// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use super::*;
impl Node {
    pub fn new_constant_getter() -> Self {
        Self::new(
            "ConstantGetter".into(),
            0,
            Box::new(
                |target_version, var_name, _input_names| match target_version {
                    TargetVersion::V0_3 => {
                        format!("let {} = make_input_getter!(ConstantGetter::new(todo!(), todo!()), T, E);\n", var_name)
                    }
                    TargetVersion::V0_4 => {
                        format!(
                            "let {} = make_input_getter(ConstantGetter::new(todo!(), todo!()));\n",
                            var_name
                        )
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!(
                            "let {} = static_reference!(ConstantGetter::new(todo!(), todo!()));\n",
                            var_name
                        )
                    }
                },
            ),
        )
    }
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
    pub fn new_expirer() -> Self {
        Self::new(
            "Expirer".into(),
            1,
            Box::new(
                |target_version, var_name, input_names: Vec<String>| match target_version {
                    TargetVersion::V0_3 => {
                        "panic!(\"streams::Expirer available in RRTK 0.4+\");\n".into()
                    }
                    TargetVersion::V0_4 => {
                        format!(
                            "let {} = make_input_getter(Expirer::new({}, todo!(), todo!()));\n",
                            var_name, input_names[0]
                        )
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!(
                            "let {} = static_reference!(Expirer::new({}, todo!(), todo!()));\n",
                            var_name, input_names[0]
                        )
                    }
                },
            ),
        )
    }
    pub fn new_latest() -> Self {
        Self::new(
            "Latest".into(),
            2,
            Box::new(
                |target_version, var_name, input_names: Vec<String>| match target_version {
                    TargetVersion::V0_3 => {
                        format!(
                            "let {} = make_input_getter!(Latest::new([{}, {}]), T, E);\n",
                            var_name, input_names[0], input_names[1]
                        )
                    }
                    TargetVersion::V0_4 => {
                        format!(
                            "let {} = make_input_getter(Latest::new([{}, {}]));\n",
                            var_name, input_names[0], input_names[1]
                        )
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!(
                            "let {} = static_reference!(Latest::new([{}, {}]));\n",
                            var_name, input_names[0], input_names[1]
                        )
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