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
    let x = Rc::new(Cell::new(100.0));
    let y = Rc::new(Cell::new(100.0));
    let drawing_area = Rc::new(DrawingArea::builder()
        .content_width(300)
        .content_height(300)
        .build());
    drawing_area.set_draw_func(clone!(@strong x, @strong y => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        println!("drawing");
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(x.get(), y.get(), 100.0, 100.0);
        context.fill().unwrap();
    }));
    let drag = GestureDrag::new();
    let click = GestureClick::new();
    drawing_area.add_controller(drag.clone());
    drawing_area.add_controller(click.clone());
    drag.connect_drag_end(drag_end);
    click.connect_pressed(clone!(@strong drawing_area => move |_gesture: &GestureClick, _click_count: i32, click_x: f64, click_y: f64| {
        x.set(click_x);
        y.set(click_y);
        drawing_area.queue_draw();
    }));
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(drawing_area.as_ref())
        .build();
    window.present();
}
fn drag_end(_gesture: &GestureDrag, x: f64, y: f64) {
    println!("drag_end {} {}", x, y);
}
