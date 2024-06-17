use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureClick, GestureDrag};
use cairo::Context;
use glib::clone;
use std::cell::Cell;
use std::rc::Rc;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let start_x = Rc::new(Cell::new(100.0));
    let start_y = Rc::new(Cell::new(100.0));
    let draw_x = Rc::new(Cell::new(100.0));
    let draw_y = Rc::new(Cell::new(100.0));
    let relative_x = Rc::new(Cell::new(0.0));
    let relative_y = Rc::new(Cell::new(0.0));
    let dragging = Rc::new(Cell::new(false));
    let drawing_area = Rc::new(DrawingArea::builder()
        .content_width(300)
        .content_height(300)
        .build());
    drawing_area.set_draw_func(clone!(@strong draw_x, @strong draw_y => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(draw_x.get(), draw_y.get(), 100.0, 100.0);
        context.fill().unwrap();
    }));
    let drag = GestureDrag::new();
    let click = GestureClick::new();
    drawing_area.add_controller(drag.clone());
    drawing_area.add_controller(click.clone());
    drag.connect_drag_end(clone!(@strong drawing_area, @strong start_x, @strong start_y, @strong draw_x, @strong draw_y, @strong relative_x, @strong relative_y, @strong dragging => move |_gesture: &GestureDrag, x: f64, y: f64| {
        if dragging.get() {
            draw_x.set(start_x.get() + relative_x.get() + x);
            draw_y.set(start_y.get() + relative_y.get() + y);
            if draw_x.get() < 0.0 {
                draw_x.set(0.0);
            } else if draw_x.get() > 200.0 {
                draw_x.set(200.0);
            }
            if draw_y.get() < 0.0 {
                draw_y.set(0.0);
            } else if draw_y.get() > 200.0 {
                draw_y.set(200.0);
            }
            drawing_area.queue_draw();
        }
    }));
    drag.connect_drag_update(clone!(@strong drawing_area, @strong start_x, @strong start_y, @strong draw_x, @strong draw_y, @strong relative_x, @strong relative_y, @strong dragging => move |_gesture: &GestureDrag, x: f64, y: f64| {
        if dragging.get() {
            draw_x.set(start_x.get() + relative_x.get() + x);
            draw_y.set(start_y.get() + relative_y.get() + y);
            if draw_x.get() < 0.0 {
                draw_x.set(0.0);
            } else if draw_x.get() > 200.0 {
                draw_x.set(200.0);
            }
            if draw_y.get() < 0.0 {
                draw_y.set(0.0);
            } else if draw_y.get() > 200.0 {
                draw_y.set(200.0);
            }
            drawing_area.queue_draw();
        }
    }));
    click.connect_pressed(clone!(@strong drawing_area, @strong relative_x, @strong relative_y, @strong dragging => move |_gesture: &GestureClick, _click_count: i32, click_x: f64, click_y: f64| {
        dragging.set(draw_x.get() <= click_x && click_x <= draw_x.get() + 100.0 && draw_y.get() <= click_y && click_y <= draw_y.get() + 100.0);
        if dragging.get() {
            start_x.set(click_x);
            start_y.set(click_y);
            relative_x.set(draw_x.get() - click_x);
            relative_y.set(draw_y.get() - click_y);
            drawing_area.queue_draw();
        }
    }));
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(drawing_area.as_ref())
        .build();
    window.present();
}
