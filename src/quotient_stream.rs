use crate::*;
#[derive(Clone)]
pub struct QuotientStreamNode {
    pub dividend_in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub divisor_in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for QuotientStreamNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self) -> String {
        let mut output = String::from(format!("let {} = make_input_getter!(QuotientStream::new(Rc::clone(&", self.get_var_name()));
        let binding = match &self.dividend_in_node {
            Some(dividend_in_node) => &dividend_in_node.borrow().get_var_name(),
            None => "dividend_input_getter",
        };
        output.push_str(binding);
        output.push_str("), Rc::clone(&");
        let binding = match &self.divisor_in_node {
            Some(divisor_in_node) => &divisor_in_node.borrow().get_var_name(),
            None => "divisor_input_getter",
        };
        output.push_str(binding);
        output.push_str(")), f32, E);\n");
        output
    }
}
