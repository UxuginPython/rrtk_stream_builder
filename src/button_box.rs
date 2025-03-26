// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use super::*;
use gtk4::Label;
pub fn make_button_box(
    push: impl Fn(Rc<RefCell<Node>>, f64, f64) -> () + Clone + 'static,
) -> gtk4::Box {
    let button_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    //This does, unfortunately, make the variable name of the button the same as the stream's
    //constructor. There is not a great way that I know of of modifying ident parameters.
    macro_rules! button {
        ($constructor:ident, $button_label:literal) => {
            let $constructor = Button::builder().label($button_label).build();
            let my_push = push.clone();
            $constructor.connect_clicked(move |_| {
                my_push(Rc::new(RefCell::new(Node::$constructor())), 100.0, 100.0)
            });
            button_box.append(&$constructor);
        };
    }
    let rrtk_label = Label::builder().label("rrtk").build();
    button_box.append(&rrtk_label);
    button!(new_constant_getter, "ConstantGetter");
    button!(new_none_getter, "NoneGetter");
    let streams_label = Label::builder().label("rrtk::streams").build();
    button_box.append(&streams_label);
    button!(new_expirer, "Expirer");
    button!(new_latest, "Latest");
    let control_label = Label::builder().label("rrtk::streams::control").build();
    button_box.append(&control_label);
    button!(new_command_pid, "CommandPID");
    button!(new_ewma_stream, "EWMAStream");
    button!(new_moving_average_stream, "MovingAverageStream");
    button!(new_pid_controller_stream, "PIDControllerStream");
    let converters_label = Label::builder().label("rrtk::streams::converters").build();
    button_box.append(&converters_label);
    button!(new_position_to_state, "PositionToState");
    button!(new_velocity_to_state, "VelocityToState");
    button!(new_acceleration_to_state, "AccelerationToState");
    button!(new_none_to_error, "NoneToError");
    button!(new_none_to_value, "NoneToValue");
    button!(new_float_to_quantity, "FloatToQuantity");
    button!(new_quantity_to_float, "QuantityToFloat");
    let flow_label = Label::builder().label("rrtk::streams::flow").build();
    button_box.append(&flow_label);
    button!(new_freeze_stream, "FreezeStream");
    button!(new_if_stream, "IfStream");
    button!(new_if_else_stream, "IfElseStream");
    let logic_label = Label::builder().label("rrtk::streams::logic").build();
    button_box.append(&logic_label);
    button!(new_and_stream, "AndStream");
    button!(new_or_stream, "OrStream");
    button!(new_not_stream, "NotStream");
    let math_label = Label::builder().label("rrtk::streams::math").build();
    button_box.append(&math_label);
    button!(new_sum_stream, "SumStream");
    button!(new_sum_2, "Sum2");
    button!(new_difference_stream, "DifferenceStream");
    button!(new_product_stream, "ProductStream");
    button!(new_product_2, "Product2");
    button!(new_quotient_stream, "QuotientStream");
    button!(new_exponent_stream, "ExponentStream");
    button!(new_derivative_stream, "DerivativeStream");
    button!(new_integral_stream, "IntegralStream");
    button_box
}
