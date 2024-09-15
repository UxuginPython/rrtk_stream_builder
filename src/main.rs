// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of RRTK Stream Builder.

    RRTK Stream Builder is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    RRTK Stream Builder is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with RRTK Stream Builder. If not, see <https://www.gnu.org/licenses/>.
*/
const NODE_WIDTH: f64 = 200.0;
const APP_ID: &str = "com.uxugin.rrtk_stream_builder";
const TARGET_VERSION: TargetVersion = TargetVersion::V0_4;
const VERSIONS: [&str; 2] = ["0.3", "0.4"];
use cairo::Context;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{
    cairo, glib, Application, ApplicationWindow, Button, DrawingArea, DropDown, GestureClick,
    GestureDrag, Orientation, Paned, ScrolledWindow, TextBuffer, TextView,
};
use std::cell::{Cell, RefCell};
use std::cmp::max;
use std::rc::Rc;
mod acceleration_to_state;
mod constant_getter;
mod derivative_stream;
mod difference_stream;
mod ewma_stream;
mod exponent_stream;
mod integral_stream;
mod latest;
mod moving_average_stream;
mod none_to_error;
mod none_to_value;
mod pid_controller_stream;
mod position_to_state;
mod product_stream;
mod quotient_stream;
mod sum_stream;
mod velocity_to_state;
use acceleration_to_state::*;
use constant_getter::*;
use derivative_stream::*;
use difference_stream::*;
use ewma_stream::*;
use exponent_stream::*;
use integral_stream::*;
use latest::*;
use moving_average_stream::*;
use none_to_error::*;
use none_to_value::*;
use pid_controller_stream::*;
use position_to_state::*;
use product_stream::*;
use quotient_stream::*;
use sum_stream::*;
use velocity_to_state::*;
enum TargetVersion {
    V0_3,
    V0_4,
}
#[derive(Clone)]
struct DragInfo {
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    action: DragAction,
}
#[derive(Clone)]
enum DragAction {
    Nothing,
    Move {
        node: Rc<RefCell<Node>>,
        relative_x: f64,
        relative_y: f64,
    },
    Connect(Rc<RefCell<Node>>),
}
#[derive(Clone, Debug)]
enum LocalTerminal {
    In(usize),
    Out,
}
#[derive(Clone, Debug)]
enum Clicked {
    Body,
    Terminal(LocalTerminal),
}
#[derive(Clone)]
enum StreamType {
    ConstantGetter,
    Latest,
    PIDControllerStream,
    EWMAStream,
    MovingAverageStream,
    NoneToError,
    NoneToValue,
    AccelerationToState,
    VelocityToState,
    PositionToState,
    SumStream,
    DifferenceStream,
    ProductStream,
    QuotientStream,
    ExponentStream,
    DerivativeStream,
    IntegralStream,
}
impl StreamType {
    fn get_in_node_count(&self) -> usize {
        match &self {
            Self::ConstantGetter => 0,
            Self::Latest => 2,
            Self::PIDControllerStream => 1,
            Self::EWMAStream => 1,
            Self::MovingAverageStream => 1,
            Self::NoneToError => 1,
            Self::NoneToValue => 1,
            Self::AccelerationToState => 1,
            Self::VelocityToState => 1,
            Self::PositionToState => 1,
            Self::SumStream => 2,
            Self::DifferenceStream => 2,
            Self::ProductStream => 2,
            Self::QuotientStream => 2,
            Self::ExponentStream => 2,
            Self::DerivativeStream => 1,
            Self::IntegralStream => 1,
        }
    }
    fn get_stream_type_string(&self) -> &str {
        match &self {
            Self::ConstantGetter => "ConstantGetter",
            Self::Latest => "Latest",
            Self::PIDControllerStream => "PIDControllerStream",
            Self::EWMAStream => "EWMAStream",
            Self::MovingAverageStream => "MovingAverageStream",
            Self::NoneToError => "NoneToError",
            Self::NoneToValue => "NoneToValue",
            Self::AccelerationToState => "AccelerationToState",
            Self::VelocityToState => "VelocityToState",
            Self::PositionToState => "PositionToState",
            Self::SumStream => "SumStream",
            Self::DifferenceStream => "DifferenceStream",
            Self::ProductStream => "ProductStream",
            Self::QuotientStream => "QuotientStream",
            Self::ExponentStream => "ExponentStream",
            Self::DerivativeStream => "DerivativeStream",
            Self::IntegralStream => "IntegralStream",
        }
    }
}
#[derive(Clone)]
struct Node {
    exists: bool,
    stream_type: StreamType,
    x: f64,
    y: f64,
    in_nodes: Vec<Option<Rc<RefCell<Node>>>>,
    converted: Option<Rc<RefCell<Box<dyn CodeGenNode>>>>,
}
impl Node {
    fn new(stream_type: StreamType, x: f64, y: f64) -> Self {
        let mut in_nodes = Vec::with_capacity(stream_type.get_in_node_count());
        for _ in 0..stream_type.get_in_node_count() {
            in_nodes.push(None);
        }
        Self {
            exists: true,
            stream_type: stream_type,
            x: x,
            y: y,
            in_nodes: in_nodes,
            converted: None,
        }
    }
    fn connect(&mut self, index: usize, node: Rc<RefCell<Node>>) {
        self.in_nodes[index] = Some(node);
    }
    fn draw(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.5, 0.5, 0.5);
        context.rectangle(
            self.x,
            self.y,
            NODE_WIDTH,
            20.0 * max(1, self.in_nodes.len()) as f64 + 10.0,
        );
        context.fill()?;
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(12.0);
        context.set_source_rgb(0.0, 0.0, 0.0);
        let extents = context
            .text_extents(self.stream_type.get_stream_type_string())
            .unwrap();
        context.move_to(
            self.x + NODE_WIDTH / 2.0 - extents.width() / 2.0,
            self.y
                + (10.0 + 20.0 * max(1, self.in_nodes.len()) as f64) / 2.0
                + extents.height() / 2.0,
        );
        context
            .show_text(self.stream_type.get_stream_type_string())
            .unwrap();
        for i in 0..self.in_nodes.len() {
            context.rectangle(self.x + 10.0, self.y + (20 * i) as f64 + 10.0, 10.0, 10.0);
        }
        context.rectangle(self.x + NODE_WIDTH - 20.0, self.y + 10.0, 10.0, 10.0);
        context.fill()?;
        Ok(())
    }
    fn draw_connections(&self, context: &Context) -> Result<(), cairo::Error> {
        context.set_source_rgb(0.0, 0.0, 0.0);
        for (i, other) in self.in_nodes.clone().into_iter().enumerate() {
            match other {
                Some(other) => {
                    context.line_to(self.x + 15.0, self.y + 20.0 * i as f64 + 15.0);
                    let (other_x, other_y) = other.borrow().get_terminal_xy(LocalTerminal::Out);
                    context.line_to(other_x, other_y);
                    context.stroke()?;
                }
                None => {}
            }
        }
        Ok(())
    }
    fn get_clicked(&self, click_x: f64, click_y: f64) -> Option<Clicked> {
        if self.x + 10.0 <= click_x && click_x <= self.x + 20.0 {
            for i in 0..self.in_nodes.len() {
                if self.y + 20.0 * i as f64 + 10.0 <= click_y
                    && click_y <= self.y + 20.0 * i as f64 + 20.0
                {
                    return Some(Clicked::Terminal(LocalTerminal::In(i)));
                }
            }
        }
        if self.x + NODE_WIDTH - 20.0 <= click_x
            && click_x <= self.x + NODE_WIDTH - 10.0
            && self.y + 10.0 <= click_y
            && click_y <= self.y + 20.0
        {
            return Some(Clicked::Terminal(LocalTerminal::Out));
        }
        if self.x <= click_x
            && click_x <= self.x + NODE_WIDTH
            && self.y <= click_y
            && click_y <= self.y + 20.0 * max(1, self.in_nodes.len()) as f64 + 10.0
        {
            return Some(Clicked::Body);
        }
        None
    }
    fn get_terminal_xy(&self, terminal: LocalTerminal) -> (f64, f64) {
        match terminal {
            LocalTerminal::In(index) => (self.x + 15.0, self.y + 15.0 + 20.0 * (index as f64)),
            LocalTerminal::Out => (self.x + NODE_WIDTH - 15.0, self.y + 15.0),
        }
    }
}
trait CodeGenNode {
    //Make this panic if it doesn't have one yet.
    fn get_var_name(&self) -> String;
    fn set_var_name(&mut self, new_var_name: String);
    fn make_line(&self, target_version: TargetVersion) -> String;
}
#[derive(Clone, Copy, Debug)]
struct NodeLoopError;
fn code_gen(nodes: Vec<Rc<RefCell<Node>>>) -> Result<String, NodeLoopError> {
    let nodes = get_existing(nodes);
    for i in &nodes {
        i.borrow_mut().converted = None;
    }
    let mut output = Vec::with_capacity(nodes.len());
    let mut last_output_len = 0usize;
    while output.len() < nodes.len() {
        for i in &nodes {
            let mut i_ref = i.borrow_mut();
            match i_ref.converted {
                Some(_) => {}
                None => {
                    let mut convertible = true;
                    let mut converted_ins = Vec::new();
                    'j: for j in &i_ref.in_nodes {
                        match j {
                            Some(in_node) => match &in_node.borrow().converted {
                                None => {
                                    convertible = false;
                                    break 'j;
                                }
                                Some(converted) => {
                                    converted_ins.push(Some(Rc::clone(&converted)));
                                }
                            },
                            None => {
                                converted_ins.push(None);
                            }
                        }
                    }
                    if convertible {
                        let converted = Rc::new(RefCell::new(match i_ref.stream_type {
                            StreamType::ConstantGetter => {
                                Box::new(ConstantGetterNode { var_name: None })
                                    as Box<dyn CodeGenNode>
                            }
                            StreamType::Latest => Box::new(LatestNode {
                                in_nodes: converted_ins,
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::PIDControllerStream => Box::new(PIDControllerStreamNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::EWMAStream => Box::new(EWMAStreamNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::MovingAverageStream => Box::new(MovingAverageStreamNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::NoneToError => Box::new(NoneToErrorNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::NoneToValue => Box::new(NoneToValueNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::AccelerationToState => Box::new(AccelerationToStateNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::VelocityToState => Box::new(VelocityToStateNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::PositionToState => Box::new(PositionToStateNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::SumStream => Box::new(SumStreamNode {
                                in_nodes: converted_ins,
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::DifferenceStream => Box::new(DifferenceStreamNode {
                                minuend_in_node: converted_ins[0].clone(),
                                subtrahend_in_node: converted_ins[1].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::ProductStream => Box::new(ProductStreamNode {
                                in_nodes: converted_ins,
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::QuotientStream => Box::new(QuotientStreamNode {
                                dividend_in_node: converted_ins[0].clone(),
                                divisor_in_node: converted_ins[1].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::ExponentStream => Box::new(ExponentStreamNode {
                                base_in_node: converted_ins[0].clone(),
                                exponent_in_node: converted_ins[1].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::DerivativeStream => Box::new(DerivativeStreamNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                            StreamType::IntegralStream => Box::new(IntegralStreamNode {
                                in_node: converted_ins[0].clone(),
                                var_name: None,
                            })
                                as Box<dyn CodeGenNode>,
                        }));
                        i_ref.converted = Some(Rc::clone(&converted));
                        output.push(converted);
                    }
                }
            }
        }
        debug_assert!(output.len() >= last_output_len);
        if output.len() == last_output_len {
            return Err(NodeLoopError);
        }
        last_output_len = output.len();
    }
    for (i, node) in output.clone().into_iter().enumerate() {
        node.borrow_mut().set_var_name(format!("node_{}", i));
    }
    let mut final_string = String::new();
    for i in output {
        final_string.push_str(&i.borrow().make_line(TARGET_VERSION));
    }
    Ok(final_string)
}
fn get_existing(nodes: Vec<Rc<RefCell<Node>>>) -> Vec<Rc<RefCell<Node>>> {
    let mut output = Vec::with_capacity(nodes.len()); //This will waste a bit of memory in some cases
                                                      //but save memory management time when pushing.
    for i in nodes {
        if i.borrow().exists {
            output.push(i);
        }
    }
    output
}
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let nodes = Rc::new(RefCell::new(Vec::new()));
    let code_gen_flag = Rc::new(Cell::new(true));
    let drag_info: Rc<RefCell<Option<RefCell<DragInfo>>>> = Rc::new(RefCell::new(None));
    let drawing_area = DrawingArea::builder()
        .width_request(1000)
        .height_request(1000)
        .build();
    let button_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    let constant_getter_button = Button::builder().label("ConstantGetter").build();
    constant_getter_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::ConstantGetter, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&constant_getter_button);
    let latest_button = Button::builder().label("Latest").build();
    latest_button.connect_clicked(
        clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
            nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::Latest, 0.0, 0.0))));
            code_gen_flag.set(true);
            drawing_area.queue_draw();
        }),
    );
    button_box.append(&latest_button);
    let pid_controller_stream_button = Button::builder().label("PIDControllerStream").build();
    pid_controller_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::PIDControllerStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&pid_controller_stream_button);
    let ewma_stream_button = Button::builder().label("EWMAStream").build();
    ewma_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::EWMAStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&ewma_stream_button);
    let moving_average_stream_button = Button::builder().label("MovingAverageStream").build();
    moving_average_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::MovingAverageStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&moving_average_stream_button);
    let none_to_error_button = Button::builder().label("NoneToError").build();
    none_to_error_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::NoneToError, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&none_to_error_button);
    let none_to_value_button = Button::builder().label("NoneToValue").build();
    none_to_value_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::NoneToValue, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&none_to_value_button);
    let acceleration_to_state_button = Button::builder().label("AccelerationToState").build();
    acceleration_to_state_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::AccelerationToState, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&acceleration_to_state_button);
    let velocity_to_state_button = Button::builder().label("VelocityToState").build();
    velocity_to_state_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::VelocityToState, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&velocity_to_state_button);
    let position_to_state_button = Button::builder().label("PositionToState").build();
    position_to_state_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::PositionToState, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&position_to_state_button);
    let sum_stream_button = Button::builder().label("SumStream").build();
    sum_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::SumStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&sum_stream_button);
    let difference_stream_button = Button::builder().label("DifferenceStream").build();
    difference_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::DifferenceStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&difference_stream_button);
    let product_stream_button = Button::builder().label("ProductStream").build();
    product_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::ProductStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&product_stream_button);
    let quotient_stream_button = Button::builder().label("QuotientStream").build();
    quotient_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::QuotientStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&quotient_stream_button);
    let exponent_stream_button = Button::builder().label("ExponentStream").build();
    exponent_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::ExponentStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&exponent_stream_button);
    let derivative_stream_button = Button::builder().label("DerivativeStream").build();
    derivative_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::DerivativeStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&derivative_stream_button);
    let integral_stream_button = Button::builder().label("IntegralStream").build();
    integral_stream_button.connect_clicked(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_| {
        nodes.borrow_mut().push(Rc::new(RefCell::new(Node::new(StreamType::IntegralStream, 0.0, 0.0))));
        code_gen_flag.set(true);
        drawing_area.queue_draw();
    }));
    button_box.append(&integral_stream_button);
    let button_box_scroll = ScrolledWindow::builder()
        .child(&button_box)
        .width_request(200)
        .build();
    let node_area = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&button_box_scroll)
        .end_child(&drawing_area)
        .width_request(1200)
        .build();
    let output_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    let target_version_selector = DropDown::from_strings(&VERSIONS);
    target_version_selector.connect_selected_notify(
        clone!(@strong target_version_selector => move |_| {
            println!("{}", target_version_selector.selected());
        }),
    );
    output_box.append(&target_version_selector);
    let text_buffer = TextBuffer::new(None);
    let text_view = TextView::builder()
        .buffer(&text_buffer)
        .monospace(true) //This doesn't seem to work?
        .editable(false)
        .vexpand(true)
        .build();
    let text_view_scroll = ScrolledWindow::builder()
        .child(&text_view)
        .width_request(700)
        .build();
    output_box.append(&text_view_scroll);
    let hor = Paned::builder()
        .orientation(Orientation::Horizontal)
        .start_child(&node_area)
        .end_child(&output_box)
        .build();
    drawing_area.set_draw_func(clone!(@strong nodes, @strong drag_info, @strong code_gen_flag => move |_drawing_area: &DrawingArea, context: &Context, _width: i32, _height: i32| {
        let nodes = get_existing((*nodes.borrow()).clone());
        for i in &nodes {
            i.borrow().draw(context).unwrap();
        }
        for i in &nodes {
            i.borrow().draw_connections(context).unwrap();
        }
        let borrow = drag_info.borrow();
        match borrow.as_ref() {
            Some(drag_info_rfcl) => {
                let drag_info_ref = drag_info_rfcl.borrow();
                match &drag_info_ref.action {
                    DragAction::Connect(node) => {
                        let (start_x, start_y) = node.borrow().get_terminal_xy(LocalTerminal::Out);
                        context.line_to(start_x, start_y);
                        context.line_to(drag_info_ref.current_x, drag_info_ref.current_y);
                        context.stroke().unwrap();
                    }
                    _ => {}
                }
            }
            None => {}
        }
        if code_gen_flag.get() {
            let code = code_gen(nodes);
            match code {
                Ok(code) => text_buffer.set_text(&code),
                Err(_) => text_buffer.set_text("error"),
            }
            code_gen_flag.set(false);
        }
    }));
    let drag = GestureDrag::new();
    let dragging_func = clone!(@strong drawing_area, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
        let borrow = drag_info.borrow();
        let drag_info_rfcl = borrow.as_ref().expect("drag_info is always Some when dragging_func is being called");
        let mut drag_info_ref = drag_info_rfcl.borrow_mut();
        drag_info_ref.current_x = drag_info_ref.start_x + x;
        drag_info_ref.current_y = drag_info_ref.start_y + y;
        match &drag_info_ref.action {
            DragAction::Move {node, relative_x, relative_y} => {
                node.borrow_mut().x = drag_info_ref.start_x + x - relative_x;
                node.borrow_mut().y = drag_info_ref.start_y + y - relative_y;
                drawing_area.queue_draw();
            }
            DragAction::Connect {..} => {
                drawing_area.queue_draw();
            }
            DragAction::Nothing => {}
        }
    });
    drag.connect_drag_end(clone!(@strong drag_info, @strong dragging_func, @strong nodes, @strong code_gen_flag => move |gesture: &GestureDrag, x: f64, y: f64| {
        dragging_func(gesture, x, y);
        let nodes = get_existing((*nodes.borrow()).clone());
        let mut drag_info_option = drag_info.borrow_mut();
        {
            let drag_info_ref = drag_info_option.as_ref().expect("drag_info is always Some when a drag is ending").borrow_mut();
            match &drag_info_ref.action {
                DragAction::Connect(node) => {
                    for i in nodes {
                        let mut i_ref = i.borrow_mut();
                        match i_ref.get_clicked(drag_info_ref.current_x, drag_info_ref.current_y) {
                            Some(Clicked::Terminal(local_terminal)) => {
                                match local_terminal {
                                    LocalTerminal::In(in_terminal) => {
                                        i_ref.connect(in_terminal, Rc::clone(&node));
                                        code_gen_flag.set(true);
                                    }
                                    LocalTerminal::Out => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        *drag_info_option = None;
    }));
    drag.connect_drag_update(dragging_func);
    drag.connect_drag_begin(clone!(@strong drawing_area, @strong nodes, @strong drag_info => move |_gesture: &GestureDrag, x: f64, y: f64| {
        let nodes = get_existing((*nodes.borrow()).clone());
        for i in nodes {
            let i_ref = i.borrow();
            match i_ref.get_clicked(x, y) {
                None => {}
                Some(clicked) => match clicked {
                    Clicked::Body => {
                        *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
                            start_x: x,
                            start_y: y,
                            current_x: x,
                            current_y: y,
                            action: DragAction::Move {
                                node: Rc::clone(&i),
                                relative_x: x - i_ref.x,
                                relative_y: y - i_ref.y,
                            }
                        }));
                        return;
                    }
                    Clicked::Terminal(local_terminal) => {
                        match local_terminal {
                            LocalTerminal::Out => {
                                *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
                                    start_x: x,
                                    start_y: y,
                                    current_x: x,
                                    current_y: y,
                                    action: DragAction::Connect(Rc::clone(&i)),
                                }));
                                return;
                            }
                            LocalTerminal::In(_) => {}
                        }
                    }
                }
            }
        }
        *drag_info.borrow_mut() = Some(RefCell::new(DragInfo {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            action: DragAction::Nothing,
        }));
    }));
    drawing_area.add_controller(drag);
    let click = GestureClick::builder().button(3).build();
    click.connect_pressed(clone!(@strong code_gen_flag, @strong drawing_area, @strong nodes => move |_, _, x, y| {
        //It will probably be faster here to just run through the nonexistant ones rather than
        //filtering them out first.
        for i in &*nodes.borrow() {
            let mut i_ref = i.borrow_mut();
            match i_ref.get_clicked(x, y) {
                Some(_) => {
                    i_ref.exists = false;
                    code_gen_flag.set(true);
                    drawing_area.queue_draw();
                }
                None => {}
            }
        }
        for i in &*nodes.borrow() {
            let mut i_ref = i.borrow_mut();
            let mut new_i_in_nodes = Vec::<Option<Rc<RefCell<Node>>>>::with_capacity(i_ref.in_nodes.len());
            for j in &i_ref.in_nodes {
                match j {
                    Some(in_node) => {
                        if in_node.borrow().exists {
                            new_i_in_nodes.push(j.clone());
                        } else {
                            new_i_in_nodes.push(None);
                        }
                    }
                    None => {
                        new_i_in_nodes.push(None);
                    }
                }
            }
            i_ref.in_nodes = new_i_in_nodes;
        }
    }));
    drawing_area.add_controller(click);
    let window = ApplicationWindow::builder()
        .application(app)
        .child(&hor)
        .title("RRTK Stream Builder")
        .build();
    window.present();
}
