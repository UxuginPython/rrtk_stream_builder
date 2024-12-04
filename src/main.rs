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
            var_name: None,
            generate_func: generate_func as Box<dyn Fn(String, Vec<String>) -> String>,
        }
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
    let quotient = Box::new(Node::new_quotient_stream());
    let none = Box::new(Node::new_none_getter());
    drag_area.push_box(quotient, 100.0, 200.0);
    drag_area.push_box(none, 100.0, 100.0);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
