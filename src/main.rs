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
    Connect,
}
#[derive(Clone, Debug)]
enum LocalTerminal {
    Left(u8),
    Right(u8),
}
#[derive(Clone, Debug)]
struct GlobalTerminal {
    pub node: usize,
    pub terminal: LocalTerminal,
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
    let nodes = Rc::new([
        Rc::new(RefCell::new(Node::new(100.0, 100.0))),
    ]);
    let drag_info = Rc::new(RefCell::new(DragInfo::Idle));
    let drawing_area = DrawingArea::builder()
        .content_width(AREA_WIDTH)
        .content_height(AREA_HEIGHT)
        .build();
    drawing_area.set_draw_func(clone!(@strong drawing_area, @strong nodes => move |drawing_area: &DrawingArea, context: &Context, width: i32, height: i32| {
        for i in &*nodes {
            i.borrow().draw(context).unwrap();
        }
    }));
    let drag = GestureDrag::new();
    let dragging_func = clone!(@strong drawing_area => move |gesture: &GestureDrag, x: f64, y: f64| {
        todo!();
    });
    drag.connect_drag_end(dragging_func);
    drag.connect_drag_update(dragging_func);
    drag.connect_drag_begin(clone!(@strong drawing_area, @strong nodes => move |gesture: &GestureDrag, x: f64, y: f64| {
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
                    todo!();
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
