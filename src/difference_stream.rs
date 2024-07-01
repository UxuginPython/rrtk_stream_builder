use crate::*;
#[derive(Clone)]
pub struct DifferenceStreamNode {
    pub minuend_in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub subtrahend_in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for DifferenceStreamNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self) -> String {
        let mut output = String::from(format!("let {} = make_input_getter!(DifferenceStream::new(Rc::clone(&", self.get_var_name()));
        let binding = match &self.minuend_in_node {
            Some(minuend_in_node) => &minuend_in_node.borrow().get_var_name(),
            None => "minuend_input_getter",
        };
        output.push_str(binding);
        output.push_str("), Rc::clone(&");
        let binding = match &self.subtrahend_in_node {
            Some(subtrahend_in_node) => &subtrahend_in_node.borrow().get_var_name(),
            None => "subtrahend_input_getter",
        };
        output.push_str(binding);
        output.push_str(")), f32, E);\n");
        output
    }
}
