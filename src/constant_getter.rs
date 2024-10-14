// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::*;
#[derive(Clone)]
pub struct ConstantGetterNode {
    pub var_name: Option<String>,
}
impl CodeGenNode for ConstantGetterNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self, target_version: TargetVersion) -> String {
        match target_version {
            TargetVersion::V0_3 => String::from(format!(
                "let {} = make_input_getter!(ConstantGetter::new(time_getter, value), G, E);\n",
                self.get_var_name()
            )),
            TargetVersion::V0_4 => String::from(format!(
                "let {} = make_input_getter(ConstantGetter::new(time_getter, value));\n",
                self.get_var_name()
            )),
        }
    }
}
