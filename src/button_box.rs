// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use super::*;
use gtk4::Button;
pub fn make_button_box(
    push: impl Fn(Rc<RefCell<Node>>, f64, f64) -> () + Clone + 'static,
) -> gtk4::Box {
    let button_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

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
