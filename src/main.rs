const AREA_WIDTH: i32 = 300;
const AREA_HEIGHT: i32 = 300;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureClick, GestureDrag};
use cairo::Context;
use glib::clone;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct DragInfo {
    pub width: f64,
    pub height: f64,
    pub start_x: f64,
    pub start_y: f64,
    pub draw_x: f64,
    pub draw_y: f64,
    pub relative_x: f64,
    pub relative_y: f64,
    pub dragging: bool,
}
impl DragInfo {
    fn new(width: f64, height: f64, x: f64, y: f64) -> DragInfo {
        DragInfo {
            width: width,
            height: height,
            start_x: x,
            start_y: y,
            draw_x: x,
            draw_y: y,
            relative_x: 0.0,
            relative_y: 0.0,
            dragging: false,
        }
    }
}
struct Thing {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub drag_info: DragInfo,
}
impl Thing {
    fn new(red: f64, green: f64, blue: f64, width: f64, height: f64, x: f64, y: f64) -> Thing {
        Thing {
            red: red,
            green: green,
            blue: blue,
            drag_info: DragInfo::new(width, height, x, y),
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
    let things = Rc::new(RefCell::new(VecDeque::from([
        Rc::new(RefCell::new(Thing::new(1.0, 0.0, 0.0, 100.0, 100.0, 0.0, 100.0))),
        Rc::new(RefCell::new(Thing::new(0.0, 1.0, 0.0, 100.0, 100.0, 100.0, 100.0))),
        Rc::new(RefCell::new(Thing::new(0.0, 0.0, 1.0, 100.0, 100.0, 200.0, 100.0))),
    ])));
    let drawing_area = Rc::new(DrawingArea::builder()
        .content_width(AREA_WIDTH)
        .content_height(AREA_HEIGHT)
        .build());
    drawing_area.set_draw_func(clone!(@strong drawing_area, @strong things => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(24.0);
        context.set_source_rgb(0.0, 0.0, 0.0);
        let extents = context.text_extents("Hello world!").unwrap();
        context.move_to((AREA_WIDTH/2) as f64 - extents.width() / 2.0, (AREA_HEIGHT/2) as f64 + extents.height() / 2.0);
        context.show_text("Hello world!").unwrap();
        let mut my_things = Vec::from(things.clone().borrow().clone());
        my_things.reverse();
        for thing in my_things {
            context.set_source_rgb(thing.borrow().red, thing.borrow().green, thing.borrow().blue);
            context.rectangle(thing.borrow().drag_info.draw_x, thing.borrow().drag_info.draw_y, thing.borrow().drag_info.width, thing.borrow().drag_info.height);
            context.fill().unwrap();
        }
    }));
    let drag = GestureDrag::new();
    let click = GestureClick::new();
    drawing_area.add_controller(drag.clone());
    drawing_area.add_controller(click.clone());
    drag.connect_drag_end(clone!(@strong drawing_area, @strong things => move |_gesture: &GestureDrag, x: f64, y: f64| {
        for thing in things.clone().borrow().clone() {
            if thing.borrow().drag_info.dragging {
                let draw_x = thing.borrow().drag_info.start_x + thing.borrow().drag_info.relative_x + x;
                let draw_y = thing.borrow().drag_info.start_y + thing.borrow().drag_info.relative_y + y;
                thing.borrow_mut().drag_info.draw_x = draw_x;
                thing.borrow_mut().drag_info.draw_y = draw_y;
                let max_x = AREA_WIDTH as f64 - thing.borrow().drag_info.width;
                let max_y = AREA_HEIGHT as f64 - thing.borrow().drag_info.height;
                if thing.borrow().drag_info.draw_x < 0.0 {
                    thing.borrow_mut().drag_info.draw_x = 0.0;
                } else if thing.borrow().drag_info.draw_x > max_x {
                    thing.borrow_mut().drag_info.draw_x = max_x;
                }
                if thing.borrow().drag_info.draw_y < 0.0 {
                    thing.borrow_mut().drag_info.draw_y = 0.0;
                } else if thing.borrow().drag_info.draw_y > max_y {
                    thing.borrow_mut().drag_info.draw_y = max_y;
                }
                drawing_area.queue_draw();
                break;
            }
        }
    }));
    drag.connect_drag_update(clone!(@strong drawing_area, @strong things => move |_gesture: &GestureDrag, x: f64, y: f64| {
        for thing in things.clone().borrow().clone() {
            if thing.borrow().drag_info.dragging {
                let draw_x = thing.borrow().drag_info.start_x + thing.borrow().drag_info.relative_x + x;
                let draw_y = thing.borrow().drag_info.start_y + thing.borrow().drag_info.relative_y + y;
                thing.borrow_mut().drag_info.draw_x = draw_x;
                thing.borrow_mut().drag_info.draw_y = draw_y;
                let max_x = AREA_WIDTH as f64 - thing.borrow().drag_info.width;
                let max_y = AREA_HEIGHT as f64 - thing.borrow().drag_info.height;
                if thing.borrow().drag_info.draw_x < 0.0 {
                    thing.borrow_mut().drag_info.draw_x = 0.0;
                } else if thing.borrow().drag_info.draw_x > max_x {
                    thing.borrow_mut().drag_info.draw_x = max_x;
                }
                if thing.borrow().drag_info.draw_y < 0.0 {
                    thing.borrow_mut().drag_info.draw_y = 0.0;
                } else if thing.borrow().drag_info.draw_y > max_y {
                    thing.borrow_mut().drag_info.draw_y = max_y;
                }
                drawing_area.queue_draw();
                break;
            }
        }
    }));
    click.connect_pressed(clone!(@strong drawing_area, @strong things => move |_gesture: &GestureClick, _click_count: i32, click_x: f64, click_y: f64| {
        let mut export = None;
        for (i, thing) in things.clone().borrow().clone().into_iter().enumerate() {
            thing.borrow_mut().drag_info.dragging = thing.borrow().drag_info.draw_x <= click_x && click_x <= thing.borrow().drag_info.draw_x + 100.0 && thing.borrow().drag_info.draw_y <= click_y && click_y <= thing.borrow().drag_info.draw_y + 100.0;
            if thing.borrow().drag_info.dragging {
                thing.borrow_mut().drag_info.start_x = click_x;
                thing.borrow_mut().drag_info.start_y = click_y;
                let relative_x = thing.borrow().drag_info.draw_x - click_x;
                let relative_y = thing.borrow().drag_info.draw_y - click_y;
                thing.borrow_mut().drag_info.relative_x = relative_x;
                thing.borrow_mut().drag_info.relative_y = relative_y;
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
