// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
#![allow(unused)]
use super::*;
fn reduce(super_path: String, name: &str, in_scope: bool) -> String {
    if in_scope {
        name.into()
    } else {
        super_path + name
    }
}
#[derive(Default)]
pub struct Crate {
    constant_getter: bool,
    none_getter: bool,
    streams: Streams,
}
impl Crate {
    pub fn string_path(&self, path: path::Crate) -> String {
        let crate_path: String = "rrtk::".into();
        match path {
            path::Crate::ConstantGetter => {
                reduce(crate_path, "ConstantGetter", self.constant_getter)
            }
            path::Crate::NoneGetter => reduce(crate_path, "NoneGetter", self.none_getter),
            path::Crate::Streams(stream_type) => self.streams.string_path(crate_path, stream_type),
        }
    }
}
#[derive(Default)]
struct Streams {
    self_in_scope: bool,
    expirer: bool,
    latest: bool,
    control: streams::Control,
    converters: streams::Converters,
    flow: streams::Flow,
    logic: streams::Logic,
    math: streams::Math,
}
impl Streams {
    fn string_path(&self, super_path: String, path: path::Streams) -> String {
        let name = reduce(super_path, "streams::", self.self_in_scope);
        match path {
            path::Streams::Expirer => reduce(name, "Expirer", self.expirer),
            path::Streams::Latest => reduce(name, "Latest", self.latest),
            path::Streams::Control(stream_type) => self.control.string_path(name, stream_type),
            path::Streams::Converters(stream_type) => {
                self.converters.string_path(name, stream_type)
            }
            path::Streams::Flow(stream_type) => self.flow.string_path(name, stream_type),
            path::Streams::Logic(stream_type) => self.logic.string_path(name, stream_type),
            path::Streams::Math(stream_type) => self.math.string_path(name, stream_type),
        }
    }
}
mod streams {
    use super::*;
    #[derive(Default)]
    pub struct Control {
        self_in_scope: bool,
        command_pid: bool,
        ewma_stream: bool,
        moving_average_stream: bool,
        pid_controller_stream: bool,
    }
    impl Control {
        pub fn string_path(&self, super_path: String, path: path::streams::Control) -> String {
            let name = reduce(super_path, "control::", self.self_in_scope);
            match path {
                path::streams::Control::CommandPID => reduce(name, "CommandPID", self.command_pid),
                path::streams::Control::EWMAStream => reduce(name, "EWMAStream", self.ewma_stream),
                path::streams::Control::MovingAverageStream => {
                    reduce(name, "MovingAverageStream", self.moving_average_stream)
                }
                path::streams::Control::PIDControllerStream => {
                    reduce(name, "PIDControllerStream", self.pid_controller_stream)
                }
            }
        }
    }
    #[derive(Default)]
    pub struct Converters {
        self_in_scope: bool,
        position_to_state: bool,
        velocity_to_state: bool,
        acceleration_to_state: bool,
        none_to_error: bool,
        none_to_value: bool,
        float_to_quantity: bool,
        quantity_to_float: bool,
    }
    impl Converters {
        pub fn string_path(&self, super_path: String, path: path::streams::Converters) -> String {
            let name = reduce(super_path, "converters::", self.self_in_scope);
            match path {
                path::streams::Converters::PositionToState => {
                    reduce(name, "PositionToState", self.position_to_state)
                }
                path::streams::Converters::VelocityToState => {
                    reduce(name, "VelocityToState", self.velocity_to_state)
                }
                path::streams::Converters::AccelerationToState => {
                    reduce(name, "AccelerationToState", self.acceleration_to_state)
                }
                path::streams::Converters::NoneToError => {
                    reduce(name, "NoneToError", self.none_to_error)
                }
                path::streams::Converters::NoneToValue => {
                    reduce(name, "NoneToValue", self.none_to_value)
                }
                path::streams::Converters::FloatToQuantity => {
                    reduce(name, "FloatToQuantity", self.float_to_quantity)
                }
                path::streams::Converters::QuantityToFloat => {
                    reduce(name, "QuantityToFloat", self.quantity_to_float)
                }
            }
        }
    }
    #[derive(Default)]
    pub struct Flow {
        self_in_scope: bool,
        freeze_stream: bool,
        if_stream: bool,
        if_else_stream: bool,
    }
    impl Flow {
        pub fn string_path(&self, super_path: String, path: path::streams::Flow) -> String {
            let name = reduce(super_path, "flow::", self.self_in_scope);
            match path {
                path::streams::Flow::FreezeStream => {
                    reduce(name, "FreezeStream", self.freeze_stream)
                }
                path::streams::Flow::IfStream => reduce(name, "IfStream", self.if_stream),
                path::streams::Flow::IfElseStream => {
                    reduce(name, "IfElseStream", self.if_else_stream)
                }
            }
        }
    }
    #[derive(Default)]
    pub struct Logic {
        self_in_scope: bool,
        and_stream: bool,
        or_stream: bool,
        not_stream: bool,
    }
    impl Logic {
        pub fn string_path(&self, super_path: String, path: path::streams::Logic) -> String {
            let name = reduce(super_path, "logic::", self.self_in_scope);
            match path {
                path::streams::Logic::AndStream => reduce(name, "AndStream", self.and_stream),
                path::streams::Logic::OrStream => reduce(name, "OrStream", self.or_stream),
                path::streams::Logic::NotStream => reduce(name, "NotStream", self.not_stream),
            }
        }
    }
    #[derive(Default)]
    pub struct Math {
        self_in_scope: bool,
        sum_stream: bool,
        sum_2: bool,
        difference_stream: bool,
        product_stream: bool,
        product_2: bool,
        quotient_stream: bool,
        exponent_stream: bool,
        derivative_stream: bool,
        integral_stream: bool,
    }
    impl Math {
        pub fn string_path(&self, super_path: String, path: path::streams::Math) -> String {
            let name = reduce(super_path, "math::", self.self_in_scope);
            match path {
                path::streams::Math::SumStream => reduce(name, "SumStream", self.sum_stream),
                path::streams::Math::Sum2 => reduce(name, "Sum2", self.sum_2),
                path::streams::Math::DifferenceStream => {
                    reduce(name, "DifferenceStream", self.difference_stream)
                }
                path::streams::Math::ProductStream => {
                    reduce(name, "ProductStream", self.product_stream)
                }
                path::streams::Math::Product2 => reduce(name, "Product2", self.product_2),
                path::streams::Math::QuotientStream => {
                    reduce(name, "QuotientStream", self.quotient_stream)
                }
                path::streams::Math::ExponentStream => {
                    reduce(name, "ExponentStream", self.exponent_stream)
                }
                path::streams::Math::DerivativeStream => {
                    reduce(name, "DerivativeStream", self.derivative_stream)
                }
                path::streams::Math::IntegralStream => {
                    reduce(name, "IntegralStream", self.integral_stream)
                }
            }
        }
    }
}
