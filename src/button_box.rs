// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use super::*;
use gtk4::{Button, Label};
pub fn make_button_box(
    push: impl Fn(Rc<RefCell<Node>>, f64, f64) -> () + Clone + 'static,
) -> gtk4::Box {
    let button_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let rrtk_label = Label::builder().label("rrtk").build();
    button_box.append(&rrtk_label);

    let constant_getter_button = Button::builder().label("ConstantGetter").build();
    let my_push = push.clone();
    constant_getter_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_constant_getter())),
            100.0,
            100.0,
        )
    });
    button_box.append(&constant_getter_button);

    let none_getter_button = Button::builder().label("NoneGetter").build();
    let my_push = push.clone();
    none_getter_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_none_getter())), 100.0, 100.0)
    });
    button_box.append(&none_getter_button);

    let streams_label = Label::builder().label("rrtk::streams").build();
    button_box.append(&streams_label);

    let expirer_button = Button::builder().label("Expirer").build();
    let my_push = push.clone();
    expirer_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_expirer())), 100.0, 100.0)
    });
    button_box.append(&expirer_button);

    let latest_button = Button::builder().label("Latest").build();
    let my_push = push.clone();
    latest_button
        .connect_clicked(move |_| my_push(Rc::new(RefCell::new(Node::new_latest())), 100.0, 100.0));
    button_box.append(&latest_button);

    let control_label = Label::builder().label("rrtk::streams::control").build();
    button_box.append(&control_label);

    let command_pid_button = Button::builder().label("CommandPID").build();
    let my_push = push.clone();
    command_pid_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_command_pid())), 100.0, 100.0)
    });
    button_box.append(&command_pid_button);

    let ewma_stream_button = Button::builder().label("EWMAStream").build();
    let my_push = push.clone();
    ewma_stream_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_ewma_stream())), 100.0, 100.0)
    });
    button_box.append(&ewma_stream_button);

    let moving_average_stream_button = Button::builder().label("MovingAverageStream").build();
    let my_push = push.clone();
    moving_average_stream_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_moving_average_stream())),
            100.0,
            100.0,
        )
    });
    button_box.append(&moving_average_stream_button);

    let pid_controller_stream_button = Button::builder().label("PIDControllerStream").build();
    let my_push = push.clone();
    pid_controller_stream_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_pid_controller_stream())),
            100.0,
            100.0,
        )
    });
    button_box.append(&pid_controller_stream_button);

    let quotient_stream_button = Button::builder().label("QuotientStream").build();
    quotient_stream_button.connect_clicked(move |_| {
        push(
            Rc::new(RefCell::new(Node::new_quotient_stream())),
            100.0,
            100.0,
        )
    });
    button_box.append(&quotient_stream_button);

    button_box
}
