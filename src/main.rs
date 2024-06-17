use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, DrawingArea};
const APP_ID: &str = "com.uxugin.gtk-cairo-test";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(256*3)
        .content_height(256*2)
        .build();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&drawing_area)
        .build();
    window.present();
}
