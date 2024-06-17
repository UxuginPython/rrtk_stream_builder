use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow, DrawingArea};
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
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&drawing_area)
        .build();
    window.present();
}
fn draw_func(_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32) {
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.line_to(100.0, 100.0);
    context.line_to(200.0, 100.0);
    context.line_to(200.0, 200.0);
    context.line_to(100.0, 200.0);
    context.line_to(100.0, 100.0);
    context.stroke().unwrap();
}
