const AREA_WIDTH: i32 = 300;
const AREA_HEIGHT: i32 = 300;
use cairo::Context;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureDrag};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
#[derive(Clone)]
struct DragInfo {
    pub start_x: f64,
    pub start_y: f64,
    pub relative_x: f64,
    pub relative_y: f64,
}
impl DragInfo {
    fn new(x: f64, y: f64) -> DragInfo {
        DragInfo {
            start_x: x,
            start_y: y,
            relative_x: 0.0,
            relative_y: 0.0,
        }
    }
}
struct Thing {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub dragging: bool,
}
impl Thing {
    fn new(red: f64, green: f64, blue: f64, width: f64, height: f64, x: f64, y: f64) -> Thing {
        Thing {
            red: red,
            green: green,
            blue: blue,
            width: width,
            height: height,
            x: x,
            y: y,
            dragging: false,
        }
    }
    fn detect_drag(&mut self, click_x: f64, click_y: f64) -> bool {
        self.dragging = self.x <= click_x
            && click_x <= self.x + self.width
            && self.y <= click_y
            && click_y <= self.y + self.height
            && !(click_x <= self.x + 20.0 && click_y <= self.y + 20.0);
        self.dragging
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(self.red, self.green, self.blue);
        context.rectangle(self.x, self.y, self.width, self.height);
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(self.x, self.y, 20.0, 20.0);
        context.fill()
    }
}
struct Node {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub dragging: bool,
    left_nodes: u8,
    right_nodes: u8,
}
impl Node {
    fn new(left_nodes: u8, right_nodes: u8, x: f64, y: f64) -> Node {
        let max_nodes = if left_nodes >= right_nodes {
            left_nodes
        } else {
            right_nodes
        };
        let height = (max_nodes as f64) * 20.0 + 10.0;
        Node {
            width: 100.0,
            height: height,
            x: x,
            y: y,
            dragging: false,
            left_nodes: left_nodes,
            right_nodes: right_nodes,
        }
    }
    fn detect_drag(&mut self, click_x: f64, click_y: f64) -> bool {
        self.dragging = self.x <= click_x
            && click_x <= self.x + self.width
            && self.y <= click_y
            && click_y <= self.y + self.height;
        for i in 0..self.left_nodes {
            if self.x + 10.0 <= click_x
                && click_x <= self.x + 20.0
                && self.y + 20.0 * (i as f64) + 10.0 <= click_y
                && click_y <= self.y + 20.0 * (i as f64) + 20.0
            {
                self.dragging = false;
            }
        }
        for i in 0..self.right_nodes {
            if self.x + self.width - 20.0 <= click_x
                && click_x <= self.x + self.width - 10.0
                && self.y + 20.0 * (i as f64) + 10.0 <= click_y
                && click_y <= self.y + 20.0 * (i as f64) + 20.0
            {
                self.dragging = false;
            }
        }
        self.dragging
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(self.x, self.y, self.width, self.height);
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        for i in 0..self.left_nodes {
            context.rectangle(self.x + 10.0, self.y + 20.0 * (i as f64) + 10.0, 10.0, 10.0);
        }
        for i in 0..self.right_nodes {
            context.rectangle(
                self.x + self.width - 20.0,
                self.y + 20.0 * (i as f64) + 10.0,
                10.0,
                10.0,
            );
        }
        context.fill()
    }
}
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let drag_info = Rc::new(RefCell::new(DragInfo::new(0.0, 0.0)));
    /*let things = Rc::new(RefCell::new(VecDeque::from([
        Rc::new(RefCell::new(Thing::new(1.0, 0.0, 0.0, 100.0, 100.0, 0.0, 100.0))),
        Rc::new(RefCell::new(Thing::new(0.0, 1.0, 0.0, 100.0, 100.0, 100.0, 100.0))),
        Rc::new(RefCell::new(Thing::new(0.0, 0.0, 1.0, 100.0, 100.0, 200.0, 100.0))),
    ])));*/
    let things = Rc::new(RefCell::new(VecDeque::from([
        Rc::new(Rc::new(RefCell::new(Node::new(1, 1, 0.0, 0.0)))),
        Rc::new(Rc::new(RefCell::new(Node::new(2, 1, 100.0, 0.0)))),
        Rc::new(Rc::new(RefCell::new(Node::new(3, 2, 200.0, 0.0)))),
    ])));
    let drawing_area = Rc::new(
        DrawingArea::builder()
            .content_width(AREA_WIDTH)
            .content_height(AREA_HEIGHT)
            .build(),
    );
    drawing_area.set_draw_func(clone!(@strong drawing_area, @strong things, @strong drag_info => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(24.0);
        context.set_source_rgb(0.0, 0.0, 0.0);
        let extents = context.text_extents("Hello world!").unwrap();
        context.move_to((AREA_WIDTH/2) as f64 - extents.width() / 2.0, (AREA_HEIGHT/2) as f64 + extents.height() / 2.0);
        context.show_text("Hello world!").unwrap();
        let mut my_things = Vec::from(things.clone().borrow().clone());
        my_things.reverse();
        for thing in my_things {
            thing.borrow().draw(context).unwrap();
        }
    }));
    let drag = GestureDrag::new();
    drawing_area.add_controller(drag.clone());
    let dragging_func = clone!(@strong drawing_area, @strong things, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
        for thing in things.clone().borrow().clone() {
            if thing.borrow().dragging {
                let draw_x = drag_info.borrow().start_x + drag_info.borrow().relative_x + x;
                let draw_y = drag_info.borrow().start_y + drag_info.borrow().relative_y + y;
                thing.borrow_mut().x = draw_x;
                thing.borrow_mut().y = draw_y;
                let max_x = AREA_WIDTH as f64 - thing.borrow().width;
                let max_y = AREA_HEIGHT as f64 - thing.borrow().height;
                if thing.borrow().x < 0.0 {
                    thing.borrow_mut().x = 0.0;
                } else if thing.borrow().x > max_x {
                    thing.borrow_mut().x = max_x;
                }
                if thing.borrow().y < 0.0 {
                    thing.borrow_mut().y = 0.0;
                } else if thing.borrow().y > max_y {
                    thing.borrow_mut().y = max_y;
                }
                drawing_area.queue_draw();
                break;
            }
        }
    });
    drag.connect_drag_end(dragging_func.clone());
    drag.connect_drag_update(dragging_func.clone());
    drag.connect_drag_begin(clone!(@strong drawing_area, @strong things, @strong drag_info => move |_gesture: &GestureDrag, click_x: f64, click_y: f64| {
        let mut export = None;
        for (i, thing) in things.clone().borrow().clone().into_iter().enumerate() {
            if thing.borrow_mut().detect_drag(click_x, click_y) {
                drag_info.borrow_mut().start_x = click_x;
                drag_info.borrow_mut().start_y = click_y;
                let relative_x = thing.borrow().x - click_x;
                let relative_y = thing.borrow().y - click_y;
                drag_info.borrow_mut().relative_x = relative_x;
                drag_info.borrow_mut().relative_y = relative_y;
                drawing_area.queue_draw();
                export = Some((i, thing));
                break;
            }
        }
        match export {
            Some((i, thing)) => {
                things.borrow_mut().remove(i);
                things.borrow_mut().push_front(thing);
            }
            None => {}
        }
    }));
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(drawing_area.as_ref())
        .build();
    window.present();
}
