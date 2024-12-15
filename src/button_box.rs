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

    let converters_label = Label::builder().label("rrtk::streams::converters").build();
    button_box.append(&converters_label);

    let position_to_state_button = Button::builder().label("PositionToState").build();
    let my_push = push.clone();
    position_to_state_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_position_to_state())),
            100.0,
            100.0,
        )
    });
    button_box.append(&position_to_state_button);

    let velocity_to_state_button = Button::builder().label("VelocityToState").build();
    let my_push = push.clone();
    velocity_to_state_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_velocity_to_state())),
            100.0,
            100.0,
        )
    });
    button_box.append(&velocity_to_state_button);

    let acceleration_to_state_button = Button::builder().label("AccelerationToState").build();
    let my_push = push.clone();
    acceleration_to_state_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_acceleration_to_state())),
            100.0,
            100.0,
        )
    });
    button_box.append(&acceleration_to_state_button);

    let none_to_error_button = Button::builder().label("NoneToError").build();
    let my_push = push.clone();
    none_to_error_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_none_to_error())),
            100.0,
            100.0,
        )
    });
    button_box.append(&none_to_error_button);

    let none_to_value_button = Button::builder().label("NoneToValue").build();
    let my_push = push.clone();
    none_to_value_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_none_to_value())),
            100.0,
            100.0,
        )
    });
    button_box.append(&none_to_value_button);

    let float_to_quantity_button = Button::builder().label("FloatToQuantity").build();
    let my_push = push.clone();
    float_to_quantity_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_float_to_quantity())),
            100.0,
            100.0,
        )
    });
    button_box.append(&float_to_quantity_button);

    let quantity_to_float_button = Button::builder().label("QuantityToFloat").build();
    let my_push = push.clone();
    quantity_to_float_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_quantity_to_float())),
            100.0,
            100.0,
        )
    });
    button_box.append(&quantity_to_float_button);

    let flow_label = Label::builder().label("rrtk::streams::flow").build();
    button_box.append(&flow_label);

    let freeze_stream_button = Button::builder().label("FreezeStream").build();
    let my_push = push.clone();
    freeze_stream_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_freeze_stream())),
            100.0,
            100.0,
        )
    });
    button_box.append(&freeze_stream_button);

    let if_stream_button = Button::builder().label("IfStream").build();
    let my_push = push.clone();
    if_stream_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_if_stream())), 100.0, 100.0)
    });
    button_box.append(&if_stream_button);

    let if_else_stream_button = Button::builder().label("IfElseStream").build();
    let my_push = push.clone();
    if_else_stream_button.connect_clicked(move |_| {
        my_push(
            Rc::new(RefCell::new(Node::new_if_else_stream())),
            100.0,
            100.0,
        )
    });
    button_box.append(&if_else_stream_button);

    let logic_label = Label::builder().label("rrtk::streams::logic").build();
    button_box.append(&logic_label);

    let and_stream_button = Button::builder().label("AndStream").build();
    let my_push = push.clone();
    and_stream_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_and_stream())), 100.0, 100.0)
    });
    button_box.append(&and_stream_button);

    let or_stream_button = Button::builder().label("OrStream").build();
    let my_push = push.clone();
    or_stream_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_or_stream())), 100.0, 100.0)
    });
    button_box.append(&or_stream_button);

    let not_stream_button = Button::builder().label("NotStream").build();
    let my_push = push.clone();
    not_stream_button.connect_clicked(move |_| {
        my_push(Rc::new(RefCell::new(Node::new_not_stream())), 100.0, 100.0)
    });
    button_box.append(&not_stream_button);

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
