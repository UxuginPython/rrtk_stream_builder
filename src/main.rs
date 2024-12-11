// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{
    cairo, glib, Application, ApplicationWindow, GestureDrag, Orientation, Paned, ScrolledWindow,
    TextBuffer, TextView,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
mod button_box;
mod node_constructors;
const NODE_WIDTH: f64 = 200.0;
const APP_ID: &str = "com.uxugin.rrtk_stream_builder";
#[derive(Clone, Copy)]
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
    x: Cell<f64>,
    y: Cell<f64>,
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
            x: Cell::new(0.0),
            y: Cell::new(0.0),
            var_name: None,
            generate_func: generate_func
                as Box<dyn Fn(TargetVersion, String, Vec<String>) -> String>,
        }
    }
    fn get_output_coords(&self) -> (f64, f64) {
        (self.x.get() + NODE_WIDTH - 15.0, self.y.get() + 15.0)
    }
    #[inline]
    fn get_draw_height(&self) -> f64 {
        max_partial_ord(30.0, 10.0 + 20.0 * self.inputs.len() as f64)
    }
    fn relative_in_output_terminal(&self, x: f64, y: f64) -> bool {
        x >= NODE_WIDTH - 20.0 && x <= NODE_WIDTH - 10.0 && y >= 10.0 && y <= 20.0
    }
    fn absolute_in_output_terminal(&self, x: f64, y: f64) -> bool {
        let (x, y) = (x - self.x.get(), y - self.y.get());
        self.relative_in_output_terminal(x, y)
    }
    fn relative_in_input_terminal(&self, x: f64, y: f64) -> Option<usize> {
        //If it's outside of the gray rectangle, None.
        if !(x >= 0.0 && x <= NODE_WIDTH && y >= 0.0 && y <= self.get_draw_height()) {
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
        let (x, y) = (x - self.x.get(), y - self.y.get());
        self.relative_in_input_terminal(x, y)
    }
}
impl Draggable for Node {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        self.x.set(x);
        self.y.set(y);
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(x, y, NODE_WIDTH, self.get_draw_height());
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(x + NODE_WIDTH - 20.0, y + 10.0, 10.0, 10.0);
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
            let (my_x, my_y) = (self.x.get() + 15.0, self.y.get() + 15.0 + 20.0 * i as f64);
            context.line_to(in_x, in_y);
            context.line_to(my_x, my_y);
            context.stroke()?;
        }
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(12.0);
        let extents = context.text_extents(&self.type_name)?;
        context.move_to(
            self.x.get() + NODE_WIDTH / 2.0 - extents.width() / 2.0,
            self.y.get() + self.get_draw_height() / 2.0 + extents.height() / 2.0,
        );
        context.show_text(&self.type_name)?;
        match &self.var_name {
            Some(var_name) => {
                context.select_font_face(
                    "Mono",
                    cairo::FontSlant::Normal,
                    cairo::FontWeight::Normal,
                );
                context.set_font_size(8.0);
                let extents = context.text_extents(&var_name)?;
                context.move_to(
                    self.x.get() + NODE_WIDTH / 2.0 - extents.width() / 2.0,
                    self.y.get() + extents.height(),
                );
                context.show_text(&var_name)?;
            }
            None => {}
        }
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (0.0, NODE_WIDTH, 0.0, self.get_draw_height())
    }
    fn contains(&self, x: f64, y: f64) -> bool {
        //If it's outside of the gray rectangle, false.
        if !(x >= 0.0 && x <= NODE_WIDTH && y >= 0.0 && y <= self.get_draw_height()) {
            return false;
        }
        //If it's in the output terminal, false.
        if x >= NODE_WIDTH - 20.0 && x <= NODE_WIDTH - 10.0 && y >= 10.0 && y <= 20.0 {
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
#[derive(Debug)]
struct NodeLoopError;
fn code_gen_one(
    node: Rc<RefCell<Node>>,
    target_version: TargetVersion,
    next_number: &mut u32,
) -> Result<String, NodeLoopError> {
    let mut node_borrow = match node.try_borrow_mut() {
        Ok(borrow) => borrow,
        Err(_) => return Err(NodeLoopError),
    };
    if node_borrow.var_name.is_some() {
        return Ok(String::new());
    }
    let mut output = String::new();
    for input in &node_borrow.inputs {
        match input {
            Some(input) => {
                output.push_str(
                    &match code_gen_one(input.clone(), target_version, next_number) {
                        Ok(string) => string,
                        Err(error) => return Err(error),
                    },
                );
            }
            None => {}
        }
    }
    node_borrow.var_name = Some(format!("node_{}", next_number));
    *next_number += 1;
    let mut input_var_names = Vec::<String>::with_capacity(node_borrow.inputs.len());
    for input in &node_borrow.inputs {
        input_var_names.push(match input {
            Some(input) => input.borrow().var_name.clone().expect("code_gen_one always leaves its input with var_name as Some, and we called it on all the inputs"),
            None => "todo!()".into(),
        });
    }
    debug_assert_eq!(input_var_names.len(), node_borrow.inputs.len());
    output.push_str(&(node_borrow.generate_func)(
        target_version,
        node_borrow.var_name.clone().unwrap(),
        input_var_names,
    ));
    Ok(output)
}
fn code_gen(
    nodes: &Vec<Rc<RefCell<Node>>>,
    target_version: TargetVersion,
) -> Result<String, NodeLoopError> {
    for node in nodes {
        node.borrow_mut().var_name = None;
    }
    let mut next_number = 0u32;
    let mut output = String::new();
    for node in nodes {
        output.push_str(
            &match code_gen_one(node.clone(), target_version, &mut next_number) {
                Ok(string) => string,
                Err(error) => return Err(error),
            },
        );
    }
    Ok(output)
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
    let drag_area = DragArea::new(1000, 1000);
    let drag_info: Rc<RefCell<Option<DragInfo>>> = Rc::new(RefCell::new(None));
    let drag_gesture_nodes = Rc::new(RefCell::new(Vec::<Rc<RefCell<Node>>>::new()));
    let text_buffer = TextBuffer::new(None);
    let text_view = TextView::builder()
        .buffer(&text_buffer)
        .monospace(true) //This doesn't seem to work?
        .editable(false)
        .vexpand(true)
        .build();
    let text_view_scroll = ScrolledWindow::builder()
        .child(&text_view)
        .width_request(700)
        .build();
    let my_drag_gesture_nodes = drag_gesture_nodes.clone();
    let code_gen_process = move || {
        text_buffer.set_text(
            &match code_gen(&my_drag_gesture_nodes.borrow(), TargetVersion::V0_6) {
                Ok(string) => string,
                Err(_) => "error".into(),
            },
        );
    };

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
    let my_code_gen_process = code_gen_process.clone();
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
        my_code_gen_process();
    });

    drag_area.add_controller(drag);
    let my_drag_area = drag_area.clone(); //This works like Rc, I think
    let my_drag_gesture_nodes = drag_gesture_nodes.clone();
    let push = move |node: Rc<RefCell<Node>>, x, y| {
        my_drag_area.push_rc_ref_cell(node.clone(), x, y);
        my_drag_gesture_nodes.borrow_mut().push(node);
        code_gen_process();
    };

    let button_box = button_box::make_button_box(push);
    let button_box_scroll = ScrolledWindow::builder()
        .child(&button_box)
        .width_request(200)
        .build();
    let node_area = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&button_box_scroll)
        .end_child(&drag_area)
        .width_request(1200)
        .build();
    let content = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&node_area)
        .end_child(&text_view_scroll)
        .build();
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&content)
        .build();
    window.present();
}
