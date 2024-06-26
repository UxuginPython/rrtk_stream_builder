const AREA_WIDTH: i32 = 500;
const AREA_HEIGHT: i32 = 500;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
use cairo::Context;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureDrag};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
#[derive(Clone, Debug)]
struct DragInfo {
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    action: DragAction,
}
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
struct Node {
    x: f64,
    y: f64,
    in_nodes: Vec<Option<Rc<RefCell<Node>>>>,
    converted: Option<Rc<RefCell<CodeGenNode>>>,
}
impl Node {
    fn new(x: f64, y: f64, in_node_count: usize) -> Self {
        let mut in_nodes = Vec::with_capacity(in_node_count);
        for _ in 0..in_node_count {
            in_nodes.push(None);
        }
        Self {
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
        context.rectangle(self.x, self.y, 50.0, 20.0 * self.in_nodes.len() as f64 + 10.0);
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        for i in 0..self.in_nodes.len() {
            context.rectangle(self.x + 10.0, self.y + (20 * i) as f64 + 10.0, 10.0, 10.0);
        }
        context.rectangle(self.x + 30.0, self.y + 10.0, 10.0, 10.0);
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
        if self.x + 50.0 - 20.0 <= click_x
            && click_x <= self.x + 50.0 - 10.0
            && self.y + 10.0 <= click_y
            && click_y <= self.y + 20.0
        {
            return Some(Clicked::Terminal(LocalTerminal::Out));
        }
        if self.x <= click_x
            && click_x <= self.x + 50.0
            && self.y <= click_y
            && click_y <= self.y + 20.0 * self.in_nodes.len() as f64 + 10.0
        {
            return Some(Clicked::Body);
        }
        None
    }
    fn get_terminal_xy(&self, terminal: LocalTerminal) -> (f64, f64) {
        match terminal {
            LocalTerminal::In(index) => (self.x + 15.0, self.y + 15.0 + 20.0 * (index as f64)),
            LocalTerminal::Out => (self.x + 50.0 - 15.0, self.y + 15.0),
        }
    }
}
#[derive(Clone, Debug)]
struct CodeGenNode {
    index: u16,
    in_nodes: Vec<Option<Rc<RefCell<CodeGenNode>>>>,
    name: Option<String>,
}
impl CodeGenNode {
    fn make_line(&self) -> String {
        let mut output = String::from(format!("let {} = make_input_getter!(FooStream::new(", self.name.clone().unwrap()));
        for i in &self.in_nodes {
            match i {
                Some(in_node) => {
                    output.push_str(&format!("Rc::clone(&{}), ", in_node.borrow().name.clone().unwrap()));
                }
                None => {
                    output.push_str("Rc::clone(&change_me), ");
                }
            }
        }
        output.pop();
        output.pop();
        output.push_str("), Foo, E);\n");
        output
    }
}
#[derive(Clone, Copy, Debug)]
struct NodeLoopError;
fn code_gen(nodes: Vec<Rc<RefCell<Node>>>) -> Result<String, NodeLoopError> {
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
                    let mut index = 0;
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
                                        let converted_index = converted.borrow().index;
                                        if converted_index + 1 > index {
                                            index = converted_index + 1;
                                        }
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
                        let converted = Rc::new(RefCell::new(CodeGenNode {
                            index: index,
                            in_nodes: converted_ins,
                            name: None,
                        }));
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
        node.borrow_mut().name = Some(format!("node_{}", i));
    }
    let mut final_string = String::new();
    for i in output {
        final_string.push_str(&i.borrow().make_line());
    }
    Ok(final_string)
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn limit(min: f64, max: f64, num: f64) -> f64 {
    let mut output = num;
    if output < min {
        output = min;
    } else if output > max {
        output = max;
    }
    output
}
fn build_ui(app: &Application) {
    let nodes = vec![
        Rc::new(RefCell::new(Node::new(50.0, 100.0, 1))),
        Rc::new(RefCell::new(Node::new(200.0, 100.0, 2))),
        Rc::new(RefCell::new(Node::new(350.0, 100.0, 2))),
    ];
    let code_gen_flag = Rc::new(Cell::new(false));
    let drag_info: Rc<RefCell<Option<RefCell<DragInfo>>>> = Rc::new(RefCell::new(None));
    let drawing_area = DrawingArea::builder()
        .content_width(AREA_WIDTH)
        .content_height(AREA_HEIGHT)
        .build();
    drawing_area.set_draw_func(clone!(@strong nodes, @strong drag_info, @strong code_gen_flag => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        for i in &*nodes {
            i.borrow().draw(context).unwrap();
        }
        for i in &*nodes {
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
            let code = code_gen(nodes.clone());
            match code {
                Ok(code) => println!("{}", code),
                Err(_) => println!("error"),
            }
            print!("\n");
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
                let height = 20.0 * node.borrow().in_nodes.len() as f64 + 10.0;
                node.borrow_mut().x = limit(0.0, (AREA_WIDTH as f64) - 50.0, drag_info_ref.start_x + x - relative_x);
                node.borrow_mut().y = limit(0.0, (AREA_HEIGHT as f64) - height, drag_info_ref.start_y + y - relative_y);
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
        let mut drag_info_option = drag_info.borrow_mut();
        {
            let drag_info_ref = drag_info_option.as_ref().expect("drag_info is always Some when a drag is ending").borrow_mut();
            match &drag_info_ref.action {
                DragAction::Connect(node) => {
                    for i in &*nodes {
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
        for i in &*nodes {
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
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drawing_area)
        .build();
    window.present();
}
