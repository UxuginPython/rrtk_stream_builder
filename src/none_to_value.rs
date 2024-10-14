// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::*;
#[derive(Clone)]
pub struct NoneToValueNode {
    pub in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for NoneToValueNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self, target_version: TargetVersion) -> String {
        match target_version {
            TargetVersion::V0_3 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter!(NoneToValue::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.in_node {
                    Some(in_node) => &in_node.borrow().get_var_name(),
                    None => "input_getter",
                };
                output.push_str(binding);
                output.push_str("), time_getter, none_value), G, E);\n");
                output
            }
            TargetVersion::V0_4 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter(NoneToValue::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.in_node {
                    Some(in_node) => &in_node.borrow().get_var_name(),
                    None => "input_getter",
                };
                output.push_str(binding);
                output.push_str("), time_getter, none_value));\n");
                output
            }
        }
    }
}
