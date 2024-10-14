use crate::*;
#[derive(Clone)]
pub struct AndStreamNode {
    pub input1: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub input2: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for AndStreamNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self, target_version: TargetVersion) -> String {
        match target_version {
            TargetVersion::V0_3 => String::from(format!(
                "let {} = panic!(\"AndStream available in RRTK 0.4\");\n",
                self.get_var_name()
            )),
            TargetVersion::V0_4 => {
                let mut output = String::from(format!(
                    "let {} = make_input_getter(AndStream::new(Rc::clone(&",
                    self.get_var_name()
                ));
                let binding = match &self.input1 {
                    Some(input1) => &input1.borrow().get_var_name(),
                    None => "input1",
                };
                output.push_str(binding);
                output.push_str("), Rc::clone(&");
                let binding = match &self.input2 {
                    Some(input2) => &input2.borrow().get_var_name(),
                    None => "input2",
                };
                output.push_str(binding);
                output.push_str(")));\n");
                output
            }
        }
    }
}
