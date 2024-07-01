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
    fn make_line(&self) -> String {
        let mut output = String::from(format!("let {} = make_input_getter!(ExponentStream::new(Rc::clone(&", self.get_var_name()));
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
}
