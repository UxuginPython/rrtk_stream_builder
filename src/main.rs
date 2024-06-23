const AREA_WIDTH: i32 = 500;
const AREA_HEIGHT: i32 = 500;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
use cairo::Context;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureDrag};
use std::cell::RefCell;
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
    Connect(GlobalOut),
}
#[derive(Clone, Debug)]
enum LocalTerminal {
    In(u8),
    Out(u8),
}
#[derive(Clone, Debug)]
struct GlobalOut {
    node: Rc<RefCell<Node>>,
    terminal: u8,
}
impl GlobalOut {
    fn get_xy(&self) -> (f64, f64) {
        self.node.borrow().get_terminal_xy(LocalTerminal::Out(self.terminal))
    }
}
#[derive(Clone, Debug)]
struct GlobalIn {
    node: Rc<RefCell<Node>>,
    terminal: u8,
}
impl GlobalIn {
    fn get_xy(&self) -> (f64, f64) {
        self.node.borrow().get_terminal_xy(LocalTerminal::In(self.terminal))
    }
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
    in_terms: u8,
    out_terms: u8,
}
impl Node {
    fn new(x: f64, y: f64, in_terms: u8, out_terms: u8) -> Self {
        Self {
            x: x,
            y: y,
            in_terms: in_terms,
            out_terms: out_terms,
        }
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        let max_terminals = if self.in_terms >= self.out_terms {
            self.in_terms
        } else {
            self.out_terms
        };
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(self.x, self.y, 50.0, (20 * max_terminals) as f64 + 10.0);
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        for i in 0..self.in_terms {
            context.rectangle(self.x + 10.0, self.y + (20 * i) as f64 + 10.0, 10.0, 10.0);
        }
        for i in 0..self.out_terms {
            context.rectangle(self.x + 30.0, self.y + (20 * i) as f64 + 10.0, 10.0, 10.0);
        }
        context.fill()?;
        Ok(())
    }
    fn get_clicked(&self, click_x: f64, click_y: f64) -> Option<Clicked> {
        if self.x + 10.0 <= click_x && click_x <= self.x + 20.0 {
            for i in 0..self.in_terms {
                if self.y + (20 * i) as f64 + 10.0 <= click_y
                    && click_y <= self.y + (20 * i) as f64 + 20.0
                {
                    return Some(Clicked::Terminal(LocalTerminal::In(i)));
                }
            }
        }
        if self.x + 50.0 - 20.0 <= click_x && click_x <= self.x + 50.0 - 10.0 {
            for i in 0..self.out_terms {
                if self.y + (20 * i) as f64 + 10.0 <= click_y
                    && click_y <= self.y + (20 * i) as f64 + 20.0
                {
                    return Some(Clicked::Terminal(LocalTerminal::Out(i)));
                }
            }
        }
        let max_terminals = if self.in_terms >= self.out_terms {
            self.in_terms
        } else {
            self.out_terms
        };
        if self.x <= click_x
            && click_x <= self.x + 50.0
            && self.y <= click_y
            && click_y <= self.y + (20 * max_terminals) as f64 + 10.0
        {
            return Some(Clicked::Body);
        }
        None
    }
    fn get_terminal_xy(&self, terminal: LocalTerminal) -> (f64, f64) {
        match terminal {
            LocalTerminal::In(index) => (self.x + 15.0, self.y + 15.0 + 20.0 * (index as f64)),
            LocalTerminal::Out(index) => {
                (self.x + 50.0 - 15.0, self.y + 15.0 + 20.0 * (index as f64))
            }
        }
    }
}
#[derive(Debug)]
struct Connection {
    start: GlobalOut,
    end: GlobalIn,
}
impl Connection {
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        let (start_x, start_y) = self.start.get_xy();
        let (end_x, end_y) = self.end.get_xy();
        context.line_to(start_x, start_y);
        context.line_to(end_x, end_y);
        context.stroke()?;
        Ok(())
    }
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
    let nodes = Rc::new(vec![
        Rc::new(RefCell::new(Node::new(50.0, 100.0, 1, 1))),
        Rc::new(RefCell::new(Node::new(200.0, 100.0, 2, 1))),
        Rc::new(RefCell::new(Node::new(350.0, 100.0, 3, 2))),
    ]);
    let connections: Rc<RefCell<Vec<Rc<RefCell<Connection>>>>> = Rc::new(RefCell::new(Vec::new()));
    let drag_info: Rc<RefCell<Option<RefCell<DragInfo>>>> = Rc::new(RefCell::new(None));
    let drawing_area = DrawingArea::builder()
        .content_width(AREA_WIDTH)
        .content_height(AREA_HEIGHT)
        .build();
    drawing_area.set_draw_func(clone!(@strong nodes, @strong drag_info, @strong connections => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        for i in &*nodes {
            i.borrow().draw(context).unwrap();
        }
        for i in &*connections.borrow() {
            i.borrow().draw(context).unwrap();
        }
        let borrow = drag_info.borrow();
        match borrow.as_ref() {
            Some(drag_info_rfcl) => {
                let drag_info_ref = drag_info_rfcl.borrow();
                match &drag_info_ref.action {
                    DragAction::Connect(global_out) => {
                        let node = &global_out.node;
                        let terminal = global_out.terminal;
                        let (start_x, start_y) = node.borrow().get_terminal_xy(LocalTerminal::Out(terminal));
                        context.line_to(start_x, start_y);
                        context.line_to(drag_info_ref.current_x, drag_info_ref.current_y);
                        context.stroke().unwrap();
                    }
                    _ => {}
                }
            }
            None => {}
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
                let max_terminals = if node.borrow().in_terms >= node.borrow().out_terms {
                    node.borrow().in_terms
                } else {
                    node.borrow().out_terms
                };
                let height = (20 * max_terminals) as f64 + 10.0;
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
    drag.connect_drag_end(clone!(@strong drag_info, @strong dragging_func, @strong nodes, @strong connections => move |gesture: &GestureDrag, x: f64, y: f64| {
        dragging_func(gesture, x, y);
        let mut drag_info_option = drag_info.borrow_mut();
        {
            let drag_info_ref = drag_info_option.as_ref().expect("drag_info is always Some when a drag is ending").borrow_mut();
            match &drag_info_ref.action {
                DragAction::Connect(global_out) => {
                    let node = &global_out.node;
                    let terminal = global_out.terminal;
                    for i in &*nodes {
                        let i_ref = i.borrow();
                        match i_ref.get_clicked(drag_info_ref.current_x, drag_info_ref.current_y) {
                            Some(Clicked::Terminal(local_terminal)) => {
                                match local_terminal {
                                    LocalTerminal::In(in_terminal) => {
                                        connections.borrow_mut().push(Rc::new(RefCell::new(Connection {
                                            start: GlobalOut {
                                                node: Rc::clone(&node),
                                                terminal: terminal,
                                            },
                                            end: GlobalIn {
                                                node: Rc::clone(&i),
                                                terminal: in_terminal,
                                            },
                                        })));
                                    }
                                    LocalTerminal::Out(_) => {}
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
                            LocalTerminal::Out(terminal) => {
                                *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
                                    start_x: x,
                                    start_y: y,
                                    current_x: x,
                                    current_y: y,
                                    action: DragAction::Connect(GlobalOut {
                                        node: Rc::clone(&i),
                                        terminal: terminal,
                                    }),
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
