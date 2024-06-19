const AREA_WIDTH: i32 = 500;
const AREA_HEIGHT: i32 = 500;
use cairo::Context;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureDrag};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
fn limit(min: f64, max: f64, num: f64) -> f64 {
    let mut output = num;
    if output < min {
        output = min;
    } else if output > max {
        output = max;
    }
    output
}
#[derive(Clone, Debug)]
enum DragInfo {
    Nothing,
    Move {
        node: usize,
        start_x: f64,
        start_y: f64,
        relative_x: f64,
        relative_y: f64,
    },
    Connect {
        start_node: usize,
        start_term: u8,
    },
}
impl DragInfo {
    fn new_move(node: usize, x: f64, y: f64) -> DragInfo {
        DragInfo::Move {
            node: node,
            start_x: x,
            start_y: y,
            relative_x: 0.0,
            relative_y: 0.0,
        }
    }
    fn new_connect(start_node: usize, start_term: u8) -> DragInfo {
        DragInfo::Connect {
            start_node: start_node,
            start_term: start_term,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Clicked {
    Nothing,
    Body,
    Left(u8),
    Right(u8),
}
#[derive(Clone)]
struct Node {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub dragging: bool,
    left_nodes: u8,
    right_nodes: u8,
    text: &'static str,
}
impl Node {
    fn new(left_nodes: u8, right_nodes: u8, x: f64, y: f64, text: &'static str) -> Node {
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
            text: text,
        }
    }
    fn clicked(&mut self, click_x: f64, click_y: f64) -> Clicked {
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
                return Clicked::Left(i);
            }
        }
        for i in 0..self.right_nodes {
            if self.x + self.width - 20.0 <= click_x
                && click_x <= self.x + self.width - 10.0
                && self.y + 20.0 * (i as f64) + 10.0 <= click_y
                && click_y <= self.y + 20.0 * (i as f64) + 20.0
            {
                self.dragging = false;
                return Clicked::Right(i);
            }
        }
        if self.dragging {
            return Clicked::Body;
        } else {
            return Clicked::Nothing;
        }
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(self.x, self.y, self.width, self.height);
        context.fill()?;
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(12.0);
        let extents = context.text_extents(self.text).unwrap();
        context.move_to(self.x + self.width / 2.0 - extents.width() / 2.0, self.y + self.height / 2.0 + extents.height() / 2.0);
        context.show_text(self.text)?;
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
struct Connection {
    start_node: usize,
    start_term: u8,
    end_node: usize,
    end_term: u8,
}
impl Connection {
    fn new(start_node: usize, start_term: u8, end_node: usize, end_term: u8) -> Connection {
        Connection {
            start_node: start_node,
            start_term: start_term,
            end_node: end_node,
            end_term: end_term,
        }
    }
}
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let drag_info = Rc::new(RefCell::new(DragInfo::Nothing));
    let things = Rc::new(RefCell::new(VecDeque::from([
        Node::new(1, 1, 0.0, 0.0, "Lorem"),
        Node::new(2, 1, 200.0, 0.0, "Ipsum"),
        Node::new(3, 2, 400.0, 0.0, "Dolor"),
    ])));
    //let connections = Rc::new(RefCell::new(Vec::new()));
    let drawing_area = Rc::new(
        DrawingArea::builder()
            .content_width(AREA_WIDTH)
            .content_height(AREA_HEIGHT)
            .build(),
    );
    drawing_area.set_draw_func(clone!(@strong drawing_area, @strong things, @strong drag_info => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        let mut my_things = Vec::from(things.clone().borrow().clone());
        my_things.reverse();
        for thing in my_things {
            thing.draw(context).unwrap();
        }
    }));
    let drag = GestureDrag::new();
    drawing_area.add_controller(drag.clone());
    let dragging_func = clone!(@strong drawing_area, @strong things, @strong drag_info => move |_gesture: &GestureDrag, drag_x: f64, drag_y: f64| {
        let mut my_things = things.borrow_mut();
        match *drag_info.borrow() {
            DragInfo::Nothing => {}
            DragInfo::Move {node, start_x, start_y, relative_x, relative_y} => {
                let draw_x = start_x + relative_x + drag_x;
                let draw_y = start_y + relative_y + drag_y;
                my_things[node].x = limit(0.0, (AREA_WIDTH as f64) - my_things[node].width, draw_x);
                my_things[node].y = limit(0.0, (AREA_HEIGHT as f64) - my_things[node].height, draw_y);
                drawing_area.queue_draw();
            }
            DragInfo::Connect {start_node, start_term} => {}
        }
    });
    /*let dragging_func = clone!(@strong drawing_area, @strong things, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
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
    });*/
    drag.connect_drag_end(dragging_func.clone());
    drag.connect_drag_update(dragging_func.clone());
    drag.connect_drag_begin(clone!(@strong drawing_area, @strong things, @strong drag_info => move |_gesture: &GestureDrag, click_x: f64, click_y: f64| {
        let mut export = None;
        for (i, mut thing) in things.clone().borrow().clone().into_iter().enumerate() {
            let clicked = thing.clicked(click_x, click_y);
            println!("{:?}", clicked);
            match clicked {
                Clicked::Body => {
                    let relative_x = thing.x - click_x;
                    let relative_y = thing.y - click_y;
                    //drag_info = DragInfo::new_move(click_x, click_y, relative_x, relative_y);
                    *drag_info.borrow_mut() = DragInfo::Move {
                        node: i,
                        start_x: click_x,
                        start_y: click_y,
                        relative_x: relative_x,
                        relative_y: relative_y,
                    };
                    drawing_area.queue_draw();
                    export = Some((i, thing));
                    break;
                }
                _ => {}
            }
        }
        match export {
            Some((i, thing)) => {
                //things.borrow_mut().remove(i);
                //things.borrow_mut().push_front(thing);
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
