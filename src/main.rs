// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, GestureDrag};
use std::{cell::RefCell, rc::Rc};
const APP_ID: &str = "com.uxugin.rrtk_stream_builder";
enum TargetVersion {
    V0_3,
    V0_4,
    V0_5,
    V0_6,
}
fn max_partial_ord<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}
struct Node {
    type_name: String,
    inputs: Vec<Option<Rc<RefCell<Node>>>>,
    x: f64,
    y: f64,
    var_name: Option<String>,
    // |my_var_name: String, input_var_names: Vec<String>| {line_of_code}
    generate_func: Box<dyn Fn(TargetVersion, String, Vec<String>) -> String>,
}
impl Node {
    fn new(
        type_name: String,
        input_count: usize,
        generate_func: Box<impl Fn(TargetVersion, String, Vec<String>) -> String + 'static>,
    ) -> Self {
        Self {
            type_name: type_name,
            inputs: vec![None; input_count],
            x: 0.0,
            y: 0.0,
            var_name: None,
            generate_func: generate_func
                as Box<dyn Fn(TargetVersion, String, Vec<String>) -> String>,
        }
    }
    fn get_output_coords(&self) -> (f64, f64) {
        (self.x + 85.0, self.y + 15.0)
    }
    #[inline]
    fn get_draw_height(&self) -> f64 {
        max_partial_ord(30.0, 10.0 + 20.0 * self.inputs.len() as f64)
    }
    fn new_none_getter() -> Self {
        Self::new(
            "NoneGetter".into(),
            0,
            Box::new(
                |target_version, var_name, _input_names| match target_version {
                    TargetVersion::V0_3 => {
                        "panic!(\"NoneGetter available in RRTK 0.4+\");\n".into()
                    }
                    TargetVersion::V0_4 => {
                        format!("let {} = make_input_getter(NoneGetter::new());\n", var_name)
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!("let {} = static_reference!(NoneGetter::new());\n", var_name)
                    }
                },
            ),
        )
    }
    fn new_quotient_stream() -> Self {
        Self::new(
            "QuotientStream".into(),
            2,
            Box::new(
                |target_version, var_name, input_names: Vec<String>| match target_version {
                    TargetVersion::V0_3 => format!(
                        "let {} = make_input_getter!(QuotientStream::new({}, {}), f32, E);\n",
                        var_name, input_names[0], input_names[1]
                    ),
                    TargetVersion::V0_4 => format!(
                        "let {} = make_input_getter(QuotientStream::new({}, {}));\n",
                        var_name, input_names[0], input_names[1]
                    ),
                    TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                        "let {} = static_reference!(QuotientStream::new({}, {}));\n",
                        var_name, input_names[0], input_names[1]
                    ),
                },
            ),
        )
    }
    fn relative_in_output_terminal(&self, x: f64, y: f64) -> bool {
        x >= 80.0 && x <= 90.0 && y >= 10.0 && y <= 20.0
    }
    fn absolute_in_output_terminal(&self, x: f64, y: f64) -> bool {
        let (x, y) = (x - self.x, y - self.y);
        self.relative_in_output_terminal(x, y)
    }
    fn relative_in_input_terminal(&self, x: f64, y: f64) -> Option<usize> {
        //If it's outside of the gray rectangle, None.
        if !(x >= 0.0 && x <= 100.0 && y >= 0.0 && y <= self.get_draw_height()) {
            return None;
        }
        //If there are no inputs, it's in the gray rectangle, and it's not in the output terminal, None.
        if self.inputs.len() == 0 {
            return None;
        }
        //If it's not in an input terminal, None.
        if !(x >= 10.0 && x <= 20.0 && y % 20.0 >= 10.0) {
            return None;
        }
        let index = (y - y % 10.0) / 10.0;
        let index = (index as usize - 1) / 2;
        Some(index)
    }
    fn absolute_in_input_terminal(&self, x: f64, y: f64) -> Option<usize> {
        let (x, y) = (x - self.x, y - self.y);
        self.relative_in_input_terminal(x, y)
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
        context.rectangle(x, y, 100.0, self.get_draw_height());
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
        (0.0, 100.0, 0.0, self.get_draw_height())
    }
    fn contains(&self, x: f64, y: f64) -> bool {
        //If it's outside of the gray rectangle, false.
        if !(x >= 0.0 && x <= 100.0 && y >= 0.0 && y <= self.get_draw_height()) {
            return false;
        }
        //If it's in the output terminal, false.
        if x >= 80.0 && x <= 90.0 && y >= 10.0 && y <= 20.0 {
            return false;
        }
        //If there are no inputs, it's in the gray rectangle, and it's not in the output terminal, true.
        if self.inputs.len() == 0 {
            return true;
        }
        //If it's in an input terminal, false.
        if x >= 10.0 && x <= 20.0 && y % 20.0 >= 10.0 {
            return false;
        }
        //If it's in the gray rectangle, it's not in the output terminal, and it's not in an input terminal, true.
        true
    }
}
#[derive(Clone)]
struct DragInfo {
    node: Rc<RefCell<Node>>,
    start_x: f64,
    start_y: f64,
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let none = Rc::new(RefCell::new(Node::new_none_getter()));
    let quotient = Rc::new(RefCell::new(Node::new_quotient_stream()));
    quotient.borrow_mut().inputs[0] = Some(Rc::clone(&none));

    let drag_area = DragArea::new(500, 500);
    let drag_info: Rc<RefCell<Option<DragInfo>>> = Rc::new(RefCell::new(None));

    let drag_gesture_nodes = Rc::new(RefCell::new(Vec::<Rc<RefCell<Node>>>::new()));
    let drag = GestureDrag::new();
    let my_drag_gesture_nodes = drag_gesture_nodes.clone();
    let my_drag_info = drag_info.clone();
    drag.connect_drag_begin(move |_gesture: &GestureDrag, x: f64, y: f64| {
        let mut my_drag_info_borrow = my_drag_info.borrow_mut();
        for node in my_drag_gesture_nodes.borrow().iter() {
            if node.borrow().absolute_in_output_terminal(x, y) {
                *my_drag_info_borrow = Some(DragInfo {
                    node: node.clone(),
                    start_x: x,
                    start_y: y,
                });
                break;
            }
        }
    });
    let my_drag_area = drag_area.clone();
    let my_drag_gesture_nodes = drag_gesture_nodes.clone();
    let my_drag_info = drag_info.clone();
    drag.connect_drag_end(move |_gesture: &GestureDrag, x: f64, y: f64| {
        let my_drag_info_borrow = match my_drag_info.borrow().clone() {
            Some(x) => x,
            None => return,
        };
        let (x, y) = (
            my_drag_info_borrow.start_x + x,
            my_drag_info_borrow.start_y + y,
        );
        for node in my_drag_gesture_nodes.borrow().iter() {
            let mut node_borrow = node.borrow_mut();
            if let Some(index) = node_borrow.absolute_in_input_terminal(x, y) {
                node_borrow.inputs[index] = Some(my_drag_info_borrow.node.clone());
                break;
            }
        }
        my_drag_area.queue_draw();
        *my_drag_info.borrow_mut() = None;
    });
    drag_area.add_controller(drag);

    let mut my_drag_area = drag_area.clone(); //This works like Rc, I think
    let my_drag_gesture_nodes = drag_gesture_nodes.clone();
    let mut push = move |node: Rc<RefCell<Node>>, x, y| {
        my_drag_area.push_rc_ref_cell(node.clone(), x, y);
        my_drag_gesture_nodes.borrow_mut().push(node);
    };

    push(none, 100.0, 100.0);
    push(quotient, 100.0, 200.0);

    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
