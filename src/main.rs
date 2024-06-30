const NODE_WIDTH: f64 = 200.0;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
use cairo::Context;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, Button, DrawingArea, GestureClick, GestureDrag, Orientation, Paned, ScrolledWindow, TextBuffer, TextView};
use std::cell::{Cell, RefCell};
use std::cmp::max;
use std::rc::Rc;
mod example;
use example::*;
#[derive(Clone)]
struct DragInfo {
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    action: DragAction,
}
#[derive(Clone)]
enum DragAction {
    Nothing,
    Move {
        node: Rc<RefCell<Node>>,
        relative_x: f64,
        relative_y: f64,
    },
    Connect(Rc<RefCell<Node>>),
}
#[derive(Clone, Debug)]
enum LocalTerminal {
    In(usize),
    Out,
}
#[derive(Clone, Debug)]
enum Clicked {
    Body,
    Terminal(LocalTerminal),
}
#[derive(Clone)]
struct Node {
    exists: bool,
    stream_type: String,
    x: f64,
    y: f64,
    in_nodes: Vec<Option<Rc<RefCell<Node>>>>,
    converted: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
}
impl Node {
    fn new(stream_type: String, x: f64, y: f64, in_node_count: usize) -> Self {
        let mut in_nodes = Vec::with_capacity(in_node_count);
        for _ in 0..in_node_count {
            in_nodes.push(None);
        }
        Self {
            exists: true,
            stream_type: stream_type,
            x: x,
            y: y,
            in_nodes: in_nodes,
            converted: None,
        }
    }
    fn connect(&mut self, index: usize, node: Rc<RefCell<Node>>) {
        self.in_nodes[index] = Some(node);
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(self.x, self.y, NODE_WIDTH, 20.0 * max(1, self.in_nodes.len()) as f64 + 10.0);
        context.fill()?;
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(12.0);
        context.set_source_rgb(0.0, 0.0, 0.0);
        let extents = context.text_extents(&self.stream_type).unwrap();
        context.move_to(self.x + NODE_WIDTH / 2.0 - extents.width() / 2.0, self.y + (10.0 + 20.0 * max(1, self.in_nodes.len()) as f64) / 2.0 + extents.height() / 2.0);
        context.show_text(&self.stream_type).unwrap();
        for i in 0..self.in_nodes.len() {
            context.rectangle(self.x + 10.0, self.y + (20 * i) as f64 + 10.0, 10.0, 10.0);
        }
        context.rectangle(self.x + NODE_WIDTH - 20.0, self.y + 10.0, 10.0, 10.0);
        context.fill()?;
        Ok(())
    }
    fn draw_connections(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.0, 0.0, 0.0);
        for (i, other) in self.in_nodes.clone().into_iter().enumerate() {
            match other {
                Some(other) => {
                    context.line_to(self.x + 15.0, self.y + 20.0 * i as f64 + 15.0);
                    let (other_x, other_y) = other.borrow().get_terminal_xy(LocalTerminal::Out);
                    context.line_to(other_x, other_y);
                    context.stroke()?;
                }
                None => {}
            }
        }
        Ok(())
    }
    fn get_clicked(&self, click_x: f64, click_y: f64) -> Option<Clicked> {
        if self.x + 10.0 <= click_x && click_x <= self.x + 20.0 {
            for i in 0..self.in_nodes.len() {
                if self.y + 20.0 * i as f64 + 10.0 <= click_y
                    && click_y <= self.y + 20.0 * i as f64 + 20.0
                {
                    return Some(Clicked::Terminal(LocalTerminal::In(i)));
                }
            }
        }
        if self.x + NODE_WIDTH - 20.0 <= click_x
            && click_x <= self.x + NODE_WIDTH - 10.0
            && self.y + 10.0 <= click_y
            && click_y <= self.y + 20.0
        {
            return Some(Clicked::Terminal(LocalTerminal::Out));
        }
        if self.x <= click_x
            && click_x <= self.x + NODE_WIDTH
            && self.y <= click_y
            && click_y <= self.y + 20.0 * max(1, self.in_nodes.len()) as f64 + 10.0
        {
            return Some(Clicked::Body);
        }
        None
    }
    fn get_terminal_xy(&self, terminal: LocalTerminal) -> (f64, f64) {
        match terminal {
            LocalTerminal::In(index) => (self.x + 15.0, self.y + 15.0 + 20.0 * (index as f64)),
            LocalTerminal::Out => (self.x + NODE_WIDTH - 15.0, self.y + 15.0),
        }
    }
}
trait CodeGenNode {
    //Make this panic if it doesn't have one yet.
    fn get_var_name(&self) -> String;
    fn set_var_name(&mut self, new_var_name: String);
    fn make_line(&self) -> String;
}
#[derive(Clone, Copy, Debug)]
struct NodeLoopError;
fn code_gen(nodes: Vec<Rc<RefCell<Node>>>) -> Result<String, NodeLoopError> {
    let nodes = get_existing(nodes);
    for i in &nodes {
        i.borrow_mut().converted = None;
    }
    let mut output = Vec::with_capacity(nodes.len());
    let mut last_output_len = 0usize;
    while output.len() < nodes.len() {
        for i in &nodes {
            let mut i_ref = i.borrow_mut();
            match i_ref.converted {
                Some(_) => {}
                None => {
                    let mut convertible = true;
                    let mut converted_ins = Vec::new();
                    'j: for j in &i_ref.in_nodes {
                        match j {
                            Some(in_node) => {
                                match &in_node.borrow().converted {
                                    None => {
                                        convertible = false;
                                        break 'j;
                                    }
                                    Some(converted) => {
                                        converted_ins.push(Some(Rc::clone(&converted)));
                                    }
                                }
                            }
                            None => {
                                converted_ins.push(None);
                            }
                        }
                    }
                    if convertible {
                        let converted = Rc::new(RefCell::new(Box::new(ExampleNode {
                            in_nodes: converted_ins,
                            var_name: None,
                        }) as Box<dyn CodeGenNode>));
                        i_ref.converted = Some(Rc::clone(&converted));
                        output.push(converted);
                    }
                }
            }
        }
        debug_assert!(output.len() >= last_output_len);
        if output.len() == last_output_len {
            return Err(NodeLoopError);
        }
        last_output_len = output.len();
    }
    for (i, node) in output.clone().into_iter().enumerate() {
        node.borrow_mut().set_var_name(format!("node_{}", i));
    }
    let mut final_string = String::new();
    for i in output {
        final_string.push_str(&i.borrow().make_line());
    }
    Ok(final_string)
}
fn get_existing(nodes: Vec<Rc<RefCell<Node>>>) -> Vec<Rc<RefCell<Node>>> {
    let mut output = Vec::with_capacity(nodes.len()); //This will waste a bit of memory in some cases
                                                      //but save memory management time when pushing.
    for i in nodes {
        if i.borrow().exists {
            output.push(i);
        }
    }
    output
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    /*let nodes = Rc::new(RefCell::new(vec![
        Rc::new(RefCell::new(Node::new("LoremStream".to_string(), 100.0, 200.0, 1))),
        Rc::new(RefCell::new(Node::new("IpsumStream".to_string(), 400.0, 200.0, 2))),
        Rc::new(RefCell::new(Node::new("DolorStream".to_string(), 700.0, 200.0, 2))),
    ]));*/
    let nodes = Rc::new(RefCell::new(Vec::new()));
    let code_gen_flag = Rc::new(Cell::new(true));
    let drag_info: Rc<RefCell<Option<RefCell<DragInfo>>>> = Rc::new(RefCell::new(None));
    let example_button = Button::builder()
        .label("ExampleStream")
        .build();
    let button_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    button_box.append(&example_button);
    let button_box_scroll = ScrolledWindow::builder()
        .child(&button_box)
        .width_request(200)
        .build();
    let drawing_area = DrawingArea::builder()
        .width_request(1000)
        .height_request(1000)
        .build();
    let node_area = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&button_box_scroll)
        .end_child(&drawing_area)
        .width_request(1200)
        .build();
    let text_buffer = TextBuffer::new(None);
    let text_view = TextView::builder()
        .buffer(&text_buffer)
        .monospace(true)
        .editable(false)
        .build();
    let text_view_scroll = ScrolledWindow::builder()
        .child(&text_view)
        .width_request(700)
        .build();
    let hor = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&node_area)
        .end_child(&text_view_scroll)
        .build();
    example_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new("ExampleStream".to_string(), 0.0, 0.0, 2))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    drawing_area.set_draw_func(clone!(@strong nodes, @strong drag_info, @strong code_gen_flag => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        let nodes = get_existing((*nodes.borrow()).clone());
        for i in &nodes {
            i.borrow().draw(context).unwrap();
        }
        for i in &nodes {
            i.borrow().draw_connections(context).unwrap();
        }
        let borrow = drag_info.borrow();
        match borrow.as_ref() {
            Some(drag_info_rfcl) => {
                let drag_info_ref = drag_info_rfcl.borrow();
                match &drag_info_ref.action {
                    DragAction::Connect(node) => {
                        let (start_x, start_y) = node.borrow().get_terminal_xy(LocalTerminal::Out);
                        context.line_to(start_x, start_y);
                        context.line_to(drag_info_ref.current_x, drag_info_ref.current_y);
                        context.stroke().unwrap();
                    }
                    _ => {}
                }
            }
            None => {}
        }
        if code_gen_flag.get() {
            let code = code_gen(nodes);
            match code {
                Ok(code) => text_buffer.set_text(&code),
                Err(_) => text_buffer.set_text("error"),
            }
            code_gen_flag.set(false);
        }
    }));
    let drag = GestureDrag::new();
    let dragging_func = clone!(@strong drawing_area, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
        let borrow = drag_info.borrow();
        let drag_info_rfcl = borrow.as_ref().expect("drag_info is always Some when dragging_func is being called");
        let mut drag_info_ref = drag_info_rfcl.borrow_mut();
        drag_info_ref.current_x = drag_info_ref.start_x + x;
        drag_info_ref.current_y = drag_info_ref.start_y + y;
        match &drag_info_ref.action {
            DragAction::Move {node, relative_x, relative_y} => {
                node.borrow_mut().x = drag_info_ref.start_x + x - relative_x;
                node.borrow_mut().y = drag_info_ref.start_y + y - relative_y;
                drawing_area.queue_draw();
            }
            DragAction::Connect {..} => {
                drawing_area.queue_draw();
            }
            DragAction::Nothing => {}
        }
    });
    drag.connect_drag_end(clone!(@strong drag_info, @strong dragging_func, @strong nodes, @strong code_gen_flag => move |gesture: &GestureDrag, x: f64, y: f64| {
        dragging_func(gesture, x, y);
        let nodes = get_existing((*nodes.borrow()).clone());
        let mut drag_info_option = drag_info.borrow_mut();
        {
            let drag_info_ref = drag_info_option.as_ref().expect("drag_info is always Some when a drag is ending").borrow_mut();
            match &drag_info_ref.action {
                DragAction::Connect(node) => {
                    for i in nodes {
                        let mut i_ref = i.borrow_mut();
                        match i_ref.get_clicked(drag_info_ref.current_x, drag_info_ref.current_y) {
                            Some(Clicked::Terminal(local_terminal)) => {
                                match local_terminal {
                                    LocalTerminal::In(in_terminal) => {
                                        i_ref.connect(in_terminal, Rc::clone(&node));
                                        code_gen_flag.set(true);
                                    }
                                    LocalTerminal::Out => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        *drag_info_option = None;
    }));
    drag.connect_drag_update(dragging_func);
    drag.connect_drag_begin(clone!(@strong drawing_area, @strong nodes, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
        let nodes = get_existing((*nodes.borrow()).clone());
        for i in nodes {
            let i_ref = i.borrow();
            match i_ref.get_clicked(x, y) {
                None => {}
                Some(clicked) => match clicked {
                    Clicked::Body => {
                        *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
                            start_x: x,
                            start_y: y,
                            current_x: x,
                            current_y: y,
                            action: DragAction::Move {
                                node: Rc::clone(&i),
                                relative_x: x - i_ref.x,
                                relative_y: y - i_ref.y,
                            }
                        }));
                        return;
                    }
                    Clicked::Terminal(local_terminal) => {
                        match local_terminal {
                            LocalTerminal::Out => {
                                *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
                                    start_x: x,
                                    start_y: y,
                                    current_x: x,
                                    current_y: y,
                                    action: DragAction::Connect(Rc::clone(&i)),
                                }));
                                return;
                            }
                            LocalTerminal::In(_) => {}
                        }
                    }
                }
            }
        }
        *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            action: DragAction::Nothing,
        }));
    }));
    drawing_area.add_controller(drag);
    let click = GestureClick::builder()
        .button(3)
        .build();
    click.connect_pressed(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_, _, x, y| {
        //It will probably be faster here to just run through the nonexistant ones rather than
        //filtering them out first.
        for i in &*nodes.borrow() {
            let mut i_ref = i.borrow_mut();
            match i_ref.get_clicked(x, y) {
                Some(_) => {
                    i_ref.exists = false;
                    code_gen_flag.set(true);
                    drawing_area.queue_draw();
                }
                None => {}
            }
        }
        for i in &*nodes.borrow() {
            let mut i_ref = i.borrow_mut();
            let mut new_i_in_nodes = Vec::<Option<Rc<RefCell<Node>>>>::with_capacity(i_ref.in_nodes.len());
            for j in &i_ref.in_nodes {
                match j {
                    Some(in_node) => {
                        if in_node.borrow().exists {
                            new_i_in_nodes.push(j.clone());
                        } else {
                            new_i_in_nodes.push(None);
                        }
                    }
                    None => {
                        new_i_in_nodes.push(None);
                    }
                }
            }
            i_ref.in_nodes = new_i_in_nodes;
        }
    }));
    drawing_area.add_controller(click);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&hor)
        .build();
    window.present();
}
