use crate::*;
#[derive(Clone)]
pub struct EWMAStreamNode {
    pub in_node: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for EWMAStreamNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self) -> String {
        let mut output = String::from(format!("let {} = make_input_getter!(EWMAStream::new(Rc::clone(&", self.get_var_name()));
        let binding = match &self.in_node {
            Some(in_node) => &in_node.borrow().get_var_name(),
            None => "input_getter",
        };
        output.push_str(binding);
        output.push_str("), smoothing_constant), f32, E);\n");
        output
    }
}
