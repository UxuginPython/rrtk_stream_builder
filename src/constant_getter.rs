use crate::*;
#[derive(Clone)]
pub struct ConstantGetterNode {
    pub var_name: Option<String>,
}
impl CodeGenNode for ConstantGetterNode {
    fn get_var_name(&self) -> String {
        self.var_name.clone().unwrap()
    }
    fn set_var_name(&mut self, new_var_name: String) {
        self.var_name = Some(new_var_name);
    }
    fn make_line(&self) -> String {
        String::from(format!("let {} = make_input_getter!(ConstantGetter::new(time_getter, value), ChangeMe, E);\n", self.get_var_name()))
    }
}
