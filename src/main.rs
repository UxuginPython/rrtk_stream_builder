use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea, GestureClick, GestureDrag};
use cairo::Context;
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(300)
        .content_height(300)
        .build();
    drawing_area.set_draw_func(draw_func);
    let drag = GestureDrag::new();
    let click = GestureClick::new();
    drawing_area.add_controller(drag.clone());
    drawing_area.add_controller(click.clone());
    drag.connect_drag_end(drag_end);
    click.connect_pressed(pressed);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&drawing_area)
        .build();
    window.present();
}
fn draw_func(_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32) {
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.rectangle(100.0, 100.0, 100.0, 100.0);
    context.fill().unwrap();
}
fn drag_end(_gesture: &GestureDrag, x: f64, y: f64) {
    println!("drag_end {} {}", x, y);
}
fn pressed(_gesture: &GestureClick, _click_count: i32, x: f64, y: f64) {
    println!("click {} {}", x, y);
}
