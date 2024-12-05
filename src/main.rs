// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow};
use std::{cell::RefCell, rc::Rc};
const APP_ID: &str = "com.uxugin.rrtk_stream_builder";
struct Node {
    type_name: String,
    inputs: Vec<Option<Rc<RefCell<Node>>>>,
    x: f64,
    y: f64,
    var_name: Option<String>,
    // |my_var_name: String, input_var_names: Vec<String>| {line_of_code}
    generate_func: Box<dyn Fn(String, Vec<String>) -> String>,
}
impl Node {
    fn new(
        type_name: String,
        input_count: usize,
        generate_func: Box<impl Fn(String, Vec<String>) -> String + 'static>,
    ) -> Self {
        Self {
            type_name: type_name,
            inputs: vec![None; input_count],
            x: 0.0,
            y: 0.0,
            var_name: None,
            generate_func: generate_func as Box<dyn Fn(String, Vec<String>) -> String>,
        }
    }
    fn get_output_coords(&self) -> (f64, f64) {
        (self.x + 85.0, self.y + 15.0)
    }
    fn new_none_getter() -> Self {
        Self::new(
            "NoneGetter".into(),
            0,
            Box::new(|var_name, _input_names| {
                format!("let {} = static_reference!(NoneGetter::new());\n", var_name)
            }),
        )
    }
    fn new_quotient_stream() -> Self {
        Self::new(
            "QuotientStream".into(),
            2,
            Box::new(|var_name, input_names: Vec<String>| {
                format!(
                    "let {} = static_reference!(QuotientStream::new({}, {}));\n",
                    var_name, input_names[0], input_names[1]
                )
            }),
        )
    }
}
fn max_partial_ord<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}
impl Draggable for Node {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        unsafe {
            //self is a reference
            let ptr = self as *const Node as *mut Node;
            (*ptr).x = x;
            (*ptr).y = y;
        }
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(
            x,
            y,
            100.0,
            max_partial_ord(30.0, 10.0 + 20.0 * self.inputs.len() as f64),
        );
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(x + 80.0, y + 10.0, 10.0, 10.0);
        for i in 0..self.inputs.len() {
            let terminal_y = y + 20.0 * i as f64 + 10.0;
            context.rectangle(x + 10.0, terminal_y, 10.0, 10.0);
        }
        context.fill()?;
        for (i, maybe_input) in self.inputs.iter().enumerate() {
            let input = match maybe_input {
                Some(x) => x.borrow(),
                None => continue,
            };
            let (in_x, in_y) = input.get_output_coords();
            let (my_x, my_y) = (self.x + 15.0, self.y + 15.0 + 20.0 * i as f64);
            context.line_to(in_x, in_y);
            context.line_to(my_x, my_y);
            context.stroke()?;
        }
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (
            0.0,
            100.0,
            0.0,
            max_partial_ord(30.0, 10.0 + 20.0 * self.inputs.len() as f64),
        )
    }
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let mut drag_area = DragArea::new(500, 500);
    let none = Rc::new(RefCell::new(Node::new_none_getter()));
    let quotient = Rc::new(RefCell::new(Node::new_quotient_stream()));
    quotient.borrow_mut().inputs[0] = Some(Rc::clone(&none));
    drag_area.push_rc_ref_cell(none, 100.0, 100.0);
    drag_area.push_rc_ref_cell(quotient, 100.0, 200.0);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
