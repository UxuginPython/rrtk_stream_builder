// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, DropDown, GestureDrag, Orientation, Paned, ScrolledWindow,
    TextBuffer, TextView, cairo, glib,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
mod button_box;
mod node_constructors;
mod path;
mod scope;
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
    if x >= y { x } else { y }
}
struct Node {
    type_name: String,
    inputs: Vec<Option<Rc<RefCell<Node>>>>,
    x: Cell<f64>,
    y: Cell<f64>,
    var_name: Option<String>,
    // |my_var_name: String, input_var_names: Vec<String>| {line_of_code}
    generate_func: Box<dyn Fn(TargetVersion, &scope::Crate, String, Vec<String>) -> String>,
    retain: Cell<bool>,
}
impl Node {
    fn new(
        type_name: String,
        input_count: usize,
        generate_func: Box<
            impl Fn(TargetVersion, &scope::Crate, String, Vec<String>) -> String + 'static,
        >,
    ) -> Self {
        Self {
            type_name: type_name,
            inputs: vec![None; input_count],
            x: Cell::new(0.0),
            y: Cell::new(0.0),
            var_name: None,
            generate_func: generate_func
                as Box<dyn Fn(TargetVersion, &scope::Crate, String, Vec<String>) -> String>,
            retain: Cell::new(true),
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
    fn can_scroll(&self, x: f64, y: f64) -> bool {
        x < 0.0 || x > NODE_WIDTH || y < 0.0 || y > self.get_draw_height()
    }
    fn retain(&self) -> bool {
        self.retain.get()
    }
    fn on_middle_click(&self) {
        self.retain.set(false);
    }
}
#[derive(Debug)]
struct NodeLoopError;
fn code_gen_one(
    node: Rc<RefCell<Node>>,
    target_version: TargetVersion,
    scope: &scope::Crate,
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
    for maybe_input in &mut node_borrow.inputs {
        match maybe_input {
            Some(input) => {
                let input_retain = match input.try_borrow() {
                    Ok(borrow) => borrow.retain.get(),
                    Err(_) => return Err(NodeLoopError),
                };
                if input_retain {
                    output.push_str(&match code_gen_one(
                        input.clone(),
                        target_version,
                        scope,
                        next_number,
                    ) {
                        Ok(string) => string,
                        Err(error) => return Err(error),
                    });
                } else {
                    *maybe_input = None;
                }
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
        scope,
        node_borrow.var_name.clone().unwrap(),
        input_var_names,
    ));
    Ok(output)
}
fn code_gen(
    nodes: &Vec<Rc<RefCell<Node>>>,
    target_version: TargetVersion,
    scope: &scope::Crate,
) -> Result<String, NodeLoopError> {
    for node in nodes {
        node.borrow_mut().var_name = None;
    }
    let mut next_number = 0u32;
    let mut output = String::new();
    for node in nodes {
        output.push_str(
            &match code_gen_one(node.clone(), target_version, scope, &mut next_number) {
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
    current_relative_x: Rc<Cell<f64>>,
    current_relative_y: Rc<Cell<f64>>,
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let drag_area = DragArea::new_scrollable(1000, 1000);
    let drag_info: Rc<RefCell<Option<DragInfo>>> = Rc::new(RefCell::new(None));
    let drag_gesture_nodes = Rc::new(RefCell::new(Vec::<Rc<RefCell<Node>>>::new()));
    let target_version = Rc::new(Cell::new(TargetVersion::V0_6));
    let scope = Rc::new(RefCell::new(scope::Crate::default()));
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
    let my_target_version = target_version.clone();
    let my_scope = scope.clone();
    let code_gen_process = move || {
        my_drag_gesture_nodes
            .borrow_mut()
            .retain(|node| node.borrow().retain());
        text_buffer.set_text(&match code_gen(
            &my_drag_gesture_nodes.borrow(),
            my_target_version.get(),
            &my_scope.borrow(),
        ) {
            Ok(string) => string,
            Err(_) => "error".into(),
        });
    };
    let my_code_gen_process = code_gen_process.clone();
    drag_area.set_pre_draw_func(move |_, _, _, _| my_code_gen_process());

    let my_drag_info = drag_info.clone();
    drag_area.set_post_draw_func(move |_, context, _, _| {
        let my_drag_info_borrow = match my_drag_info.borrow().clone() {
            Some(x) => x,
            None => return,
        };
        let (start_x, start_y) = (my_drag_info_borrow.start_x, my_drag_info_borrow.start_y);
        let (end_x, end_y) = (
            start_x + my_drag_info_borrow.current_relative_x.get(),
            start_y + my_drag_info_borrow.current_relative_y.get(),
        );
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.move_to(start_x, start_y);
        context.line_to(end_x, end_y);
        context.stroke().unwrap();
    });

    let target_version_selector = DropDown::from_strings(&["0.3", "0.4", "0.5", "0.6"]);
    target_version_selector.set_selected(3);
    let my_target_version_selector = target_version_selector.clone();
    let my_code_gen_process = code_gen_process.clone();
    target_version_selector.connect_selected_notify(move |_| {
        target_version.set(match my_target_version_selector.selected() {
            0 => TargetVersion::V0_3,
            1 => TargetVersion::V0_4,
            2 => TargetVersion::V0_5,
            3 => TargetVersion::V0_6,
            _ => panic!("impossible value from target_version_selector"),
        });
        my_code_gen_process();
    });
    let output_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    output_box.append(&target_version_selector);
    output_box.append(&text_view_scroll);

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
                    current_relative_x: Rc::new(Cell::new(0.0)),
                    current_relative_y: Rc::new(Cell::new(0.0)),
                });
                break;
            }
        }
    });

    let my_drag_area = drag_area.clone();
    let my_drag_info = drag_info.clone();
    drag.connect_drag_update(move |_gesture: &GestureDrag, x: f64, y: f64| {
        let my_drag_info_borrow = match my_drag_info.borrow_mut().clone() {
            Some(x) => x,
            None => return,
        };
        my_drag_info_borrow.current_relative_x.set(x);
        my_drag_info_borrow.current_relative_y.set(y);
        my_drag_area.queue_draw();
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
        .end_child(&output_box)
        .build();
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&content)
        .title("RRTK Stream Builder")
        .build();
    window.present();
}
