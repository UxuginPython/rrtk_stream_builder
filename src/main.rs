const AREA_WIDTH: i32 = 500;
const AREA_HEIGHT: i32 = 500;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, cairo, DrawingArea, GestureDrag, glib};
use cairo::Context;
use glib::clone;
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
    let drawing_area = DrawingArea::builder()
        .content_width(AREA_WIDTH)
        .content_height(AREA_HEIGHT)
        .build();
    drawing_area.set_draw_func(clone!(@strong drawing_area => move |drawing_area: &DrawingArea, context: &Context, width: i32, height: i32| {
        todo!();
    }));
    let drag = GestureDrag::new();
    let dragging_func = clone!(@strong drawing_area => move |gesture: &GestureDrag, x: f64, y: f64| {
        todo!();
    });
    drag.connect_drag_end(dragging_func);
    drag.connect_drag_update(dragging_func);
    drag.connect_drag_begin(clone!(@strong drawing_area => move |gesture: &GestureDrag, x: f64, y: f64| {
        todo!();
    }));
    drawing_area.add_controller(drag);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drawing_area)
        .build();
    window.present();
}
