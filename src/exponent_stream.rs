// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of RRTK Stream Builder.

    RRTK Stream Builder is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    RRTK Stream Builder is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with RRTK Stream Builder. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::*;
#[derive(Clone)]
pub struct ExponentStreamNode {
    pub base_in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub exponent_in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for ExponentStreamNode {
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
                    "let {} = make_input_getter!(ExponentStream::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.base_in_node {
                    Some(base_in_node) => &base_in_node.borrow().get_var_name(),
                    None => "base_input_getter",
                };
                output.push_str(binding);
                output.push_str("), Rc::clone(&");
                let binding = match &self.exponent_in_node {
                    Some(exponent_in_node) => &exponent_in_node.borrow().get_var_name(),
                    None => "exponent_input_getter",
                };
                output.push_str(binding);
                output.push_str(")), f32, E);\n");
                output
            }
            TargetVersion::V0_4 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter(ExponentStream::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.base_in_node {
                    Some(base_in_node) => &base_in_node.borrow().get_var_name(),
                    None => "base_input_getter",
                };
                output.push_str(binding);
                output.push_str("), Rc::clone(&");
                let binding = match &self.exponent_in_node {
                    Some(exponent_in_node) => &exponent_in_node.borrow().get_var_name(),
                    None => "exponent_input_getter",
                };
                output.push_str(binding);
                output.push_str(")));\n");
                output
            }
        }
    }
}
