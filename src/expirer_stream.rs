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
pub struct ExpirerNode {
    pub in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for ExpirerNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self, target_version: TargetVersion) -> String {
        match target_version {
            TargetVersion::V0_3 => String::from(format!(
                "let {} = panic!(\"Expirer available in RRTK 0.4\");\n",
                self.get_var_name()
            )),
            TargetVersion::V0_4 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter(Expirer::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.in_node {
                    Some(in_node) => &in_node.borrow().get_var_name(),
                    None => "input_getter",
                };
                output.push_str(binding);
                output.push_str("), time_getter, max_time_delta));\n");
                output
            }
        }
    }
}
