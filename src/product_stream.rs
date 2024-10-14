// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::*;
#[derive(Clone)]
pub struct ProductStreamNode {
    pub in_nodes: Vec<Option<Rc<RefCell<Box<dyn CodeGenNode>>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for ProductStreamNode {
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
                    "let {} = make_input_getter!(ProductStream::new([",
                    self.get_var_name()
                ));
                let mut pop = false;
                for i in &self.in_nodes {
                    match i {
                        Some(in_node) => {
                            output.push_str(&format!(
                                "Rc::clone(&{}), ",
                                in_node.borrow().get_var_name()
                            ));
                            pop = true;
                        }
                        None => {
                            output.push_str("Rc::clone(&input_getter), ");
                            pop = true;
                        }
                    }
                }
                if pop {
                    output.pop();
                    output.pop();
                }
                output.push_str("]), f32, E);\n");
                output
            }
            TargetVersion::V0_4 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter(ProductStream::new([",
                    self.get_var_name()
                ));
                let mut pop = false;
                for i in &self.in_nodes {
                    match i {
                        Some(in_node) => {
                            output.push_str(&format!(
                                "Rc::clone(&{}), ",
                                in_node.borrow().get_var_name()
                            ));
                            pop = true;
                        }
                        None => {
                            output.push_str("Rc::clone(&input_getter), ");
                            pop = true;
                        }
                    }
                }
                if pop {
                    output.pop();
                    output.pop();
                }
                output.push_str("]));\n");
                output
            }
        }
    }
}
