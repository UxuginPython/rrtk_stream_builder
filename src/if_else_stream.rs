// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::*;
#[derive(Clone)]
pub struct IfElseStreamNode {
    pub condition: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub true_out: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub false_out: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for IfElseStreamNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self, target_version: TargetVersion) -> String {
        match target_version {
            TargetVersion::V0_3 => String::from(format!(
                "let {} = panic!(\"IfElseStream available in RRTK 0.4\");\n",
                self.get_var_name()
            )),
            TargetVersion::V0_4 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter(IfElseStream::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.condition {
                    Some(condition) => &condition.borrow().get_var_name(),
                    None => "condition_getter",
                };
                output.push_str(binding);
                output.push_str("), Rc::clone(&");
                let binding = match &self.true_out {
                    Some(true_out) => &true_out.borrow().get_var_name(),
                    None => "true_getter",
                };
                output.push_str(binding);
                output.push_str("), Rc::clone(&");
                let binding = match &self.false_out {
                    Some(false_out) => &false_out.borrow().get_var_name(),
                    None => "false_getter",
                };
                output.push_str(binding);
                output.push_str(")));\n");
                output
            }
        }
    }
}
