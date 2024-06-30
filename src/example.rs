use crate::*;
#[derive(Clone)]
pub struct ExampleNode {
    pub in_nodes: Vec<Option<Rc<RefCell<Box<dyn CodeGenNode>>>>>,
    pub var_name: Option<String>,
}
impl CodeGenNode for ExampleNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self) -> String {
        let mut output = String::from(format!("let {} = make_input_getter!(ExampleStream::new(", self.get_var_name()));
        let mut pop = false;
        for i in &self.in_nodes {
            match i {
                Some(in_node) => {
                    output.push_str(&format!("Rc::clone(&{}), ", in_node.borrow().get_var_name()));
                    pop = true;
                }
                None => {
                    output.push_str("Rc::clone(&change_me), ");
                    pop = true;
                }
            }
        }
        if pop {
            output.pop();
            output.pop();
        }
        output.push_str("), Example, E);\n");
        output
    }
}
