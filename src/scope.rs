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
pub struct Crate {
    constant_getter: bool,
    none_getter: bool,
    streams: Streams,
}
impl Crate {
    pub fn new() -> Self {
        Self {
            constant_getter: false,
            none_getter: false,
            streams: Streams::new(),
        }
    }
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
struct Streams {
    self_in_scope: bool,
    expirer: bool,
    latest: bool,
    control: streams::Control,
    converters: streams::Converters,
    flow: streams::Flow,
}
impl Streams {
    fn new() -> Self {
        Self {
            self_in_scope: false,
            expirer: false,
            latest: false,
            control: streams::Control::new(),
            converters: streams::Converters::new(),
            flow: streams::Flow::new(),
        }
    }
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
        }
    }
}
mod streams {
    use super::*;
    pub struct Control {
        self_in_scope: bool,
        command_pid: bool,
        ewma_stream: bool,
        moving_average_stream: bool,
        pid_controller_stream: bool,
    }
    impl Control {
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
                command_pid: false,
                ewma_stream: false,
                moving_average_stream: false,
                pid_controller_stream: false,
            }
        }
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
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
                position_to_state: false,
                velocity_to_state: false,
                acceleration_to_state: false,
                none_to_error: false,
                none_to_value: false,
                float_to_quantity: false,
                quantity_to_float: false,
            }
        }
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
    pub struct Flow {
        self_in_scope: bool,
        freeze_stream: bool,
        if_stream: bool,
        if_else_stream: bool,
    }
    impl Flow {
        pub fn new() -> Self {
            Self {
                self_in_scope: false,
                freeze_stream: false,
                if_stream: false,
                if_else_stream: false,
            }
        }
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
}
