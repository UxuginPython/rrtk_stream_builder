// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use cairo::{Context, Error};
use cairodrag::*;
use gtk4::prelude::*;
use gtk4::{cairo, glib, Application, ApplicationWindow};
const APP_ID: &str = "com.uxugin.rrtk_stream_builder";
trait Node {
    fn get_input_count(&self) -> usize;
    fn get_type_name(&self) -> String; //or &str?
    fn set_var_name(&mut self, new_var_name: String);
    fn get_var_name(&self) -> Option<String>;
    fn get_generate_ready(&self) -> bool;
    fn get_generated_code(&self) -> Option<String>;
    fn reset(&mut self);
}
impl<T: Node> Draggable for T {
    fn draw(&self, context: &Context, x: f64, y: f64) -> Result<(), Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(x, y, 100.0, 10.0 + 20.0 * self.get_input_count() as f64);
        context.fill()?;
        Ok(())
    }
    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (0.0, 100.0, 0.0, 10.0 + 20.0 * self.get_input_count() as f64)
    }
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let mut drag_area = DragArea::new(500, 500);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&drag_area)
        .build();
    window.present();
}
