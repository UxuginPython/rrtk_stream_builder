const AREA_WIDTH: i32 = 500;
const AREA_HEIGHT: i32 = 500;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, cairo, DrawingArea, GestureDrag, glib};
use cairo::Context;
use glib::clone;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Clone, Debug)]
enum DragInfo {
    Idle,
    Move {
        node: Rc<RefCell<Node>>,
        start_x: f64,
        start_y: f64,
        relative_x: f64,
        relative_y: f64,
    },
    Connect {
        terminal: GlobalTerminal,
        draw_x: f64,
        draw_y: f64,
    },
}
#[derive(Clone, Copy, Debug)]
enum LocalTerminal {
    Left(u8),
    Right(u8),
}
#[derive(Clone, Debug)]
struct GlobalTerminal {
    pub node: Rc<RefCell<Node>>,
    pub terminal: LocalTerminal,
}
impl GlobalTerminal {
    fn new(node: Rc<RefCell<Node>>, terminal: LocalTerminal) -> Self {
        Self {
            node: node,
            terminal: terminal,
        }
    }
    fn get_xy(&self) -> (f64, f64) {
        self.node.borrow().get_terminal_xy(self.terminal)
    }
}
#[derive(Clone, Debug)]
enum Clicked {
    Nothing,
    Body,
    Terminal(LocalTerminal),
}
#[derive(Clone, Debug)]
struct Node {
    x: f64,
    y: f64,
}
impl Node {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(self.x, self.y, 50.0, 30.0);
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(self.x + 10.0, self.y + 10.0, 10.0, 10.0);
        context.rectangle(self.x + 30.0, self.y + 10.0, 10.0, 10.0);
        context.fill()?;
        Ok(())
    }
    fn get_clicked(&self, click_x: f64, click_y: f64) -> Clicked {
        if self.x + 10.0 <= click_x
            && click_x <= self.x + 20.0
            && self.y + 10.0 <= click_y
            && click_y <= self.x + 20.0 {
            return Clicked::Terminal(LocalTerminal::Left(0));
        } else if self.x + 30.0 <= click_x
            && click_x <= self.x + 40.0
            && self.y + 10.0 <= click_y
            && click_y <= self.x + 20.0 {
            return Clicked::Terminal(LocalTerminal::Right(0));
        } else if self.x <= click_x
            && click_x <= self.x + 50.0
            && self.y <= click_y
            && click_y <= self.y + 30.0 {
            return Clicked::Body;
        }
        Clicked::Nothing
    }
    fn get_terminal_xy(&self, terminal: LocalTerminal) -> (f64, f64) {
        match terminal {
            LocalTerminal::Left(index) => (self.x + 15.0, self.y + 20.0 * (index as f64) + 15.0),
            LocalTerminal::Right(index) => (self.x + 50.0 - 15.0, self.y + 20.0 * (index as f64) + 15.0),
        }
    }
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
/*fn limit(min: f64, max: f64, num: f64) -> f64 {
    let mut output = num;
    if output < min {
        output = min;
    } else if output > max {
        output = max;
    }
    output
}*/
fn build_ui(app: &Application) {
    let nodes = Rc::new([
        Rc::new(RefCell::new(Node::new(100.0, 100.0))),
    ]);
    let drag_info = Rc::new(RefCell::new(DragInfo::Idle));
    let drawing_area = DrawingArea::builder()
        .content_width(AREA_WIDTH)
        .content_height(AREA_HEIGHT)
        .build();
    drawing_area.set_draw_func(clone!(@strong nodes, @strong drag_info => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        for i in &*nodes {
            i.borrow().draw(context).unwrap();
        }
        match drag_info.borrow().clone() {
            DragInfo::Connect {terminal, draw_x, draw_y} => {
                let (term_x, term_y) = terminal.get_xy();
                context.set_source_rgb(0.0, 0.0, 0.0);
                context.line_to(term_x, term_y);
                context.line_to(draw_x, draw_y);
                println!("({:?}, {:?}) -> ({:?}, {:?})", term_x, term_y, draw_x, draw_y);
                context.stroke().unwrap();
            }
            _ => {}
        }
    }));
    let drag = GestureDrag::new();
    let dragging_func = clone!(@strong drawing_area, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
        match drag_info.borrow().clone() {
            DragInfo::Idle => {}
            DragInfo::Move {node, start_x, start_y, relative_x, relative_y} => {
                node.borrow_mut().x = start_x - relative_x + x;
                node.borrow_mut().y = start_y - relative_y + y;
                drawing_area.queue_draw();
            }
            DragInfo::Connect {terminal, mut draw_x, mut draw_y} => {
                println!("dragging_func ({:?}, {:?})", x, y);
                let (term_x, term_y) = terminal.get_xy();
                draw_x = term_x + x;
                draw_y = term_y + y;
                drawing_area.queue_draw();
            }
        }
    });
    drag.connect_drag_end(clone!(@strong drag_info, @strong dragging_func => move |gesture: &GestureDrag, width: f64, height: f64| {
        dragging_func(gesture, width, height);
        *drag_info.borrow_mut() = DragInfo::Idle;
    }));
    drag.connect_drag_update(dragging_func);
    drag.connect_drag_begin(clone!(@strong drawing_area, @strong nodes => move |_gesture: &GestureDrag, x: f64, y: f64| {
        for i in &*nodes {
            match i.borrow().get_clicked(x, y) {
                Clicked::Nothing => {}
                Clicked::Body => {
                    *drag_info.borrow_mut() = DragInfo::Move {
                        node: Rc::clone(i),
                        start_x: x,
                        start_y: y,
                        relative_x: x - i.borrow().x,
                        relative_y: y - i.borrow().y,
                    }
                }
                Clicked::Terminal(local_terminal) => {
                    *drag_info.borrow_mut() = DragInfo::Connect {
                        terminal: GlobalTerminal::new(i.clone(), local_terminal),
                        draw_x: x,
                        draw_y: y,
                    };
                }
            }
        }
    }));
    drawing_area.add_controller(drag);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drawing_area)
        .build();
    window.present();
}
