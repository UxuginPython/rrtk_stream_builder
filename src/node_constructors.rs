// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use super::*;
impl Node {
    pub fn new_constant_getter() -> Self {
        Self::new(
            "ConstantGetter".into(),
            0,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, _input_names| match target_version
                {
                    TargetVersion::V0_3 => {
                        format!(
                            "let {} = make_input_getter!({}::new(todo!(), todo!()), T, E);\n",
                            var_name,
                            scope.string_path(path::Crate::ConstantGetter),
                        )
                    }
                    TargetVersion::V0_4 => {
                        format!(
                            "let {} = make_input_getter({}::new(todo!(), todo!()));\n",
                            var_name,
                            scope.string_path(path::Crate::ConstantGetter),
                        )
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!(
                            "let {} = static_reference!({}::new(todo!(), todo!()));\n",
                            var_name,
                            scope.string_path(path::Crate::ConstantGetter),
                        )
                    }
                },
            ),
        )
    }
    pub fn new_none_getter() -> Self {
        Self::new(
            "NoneGetter".into(),
            0,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, _input_names| match target_version
                {
                    TargetVersion::V0_3 => {
                        format!(
                            "let {} = panic!(\"NoneGetter available in RRTK 0.4+\");\n",
                            var_name
                        )
                    }
                    TargetVersion::V0_4 => {
                        format!(
                            "let {} = make_input_getter({}::new());\n",
                            var_name,
                            scope.string_path(path::Crate::NoneGetter)
                        )
                    }
                    TargetVersion::V0_5 | TargetVersion::V0_6 => {
                        format!(
                            "let {} = static_reference!({}::new());\n",
                            var_name,
                            scope.string_path(path::Crate::NoneGetter)
                        )
                    }
                },
            ),
        )
    }
    pub fn new_expirer() -> Self {
        Self::new(
            "Expirer".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"streams::Expirer available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Expirer)),
                                input_names[0]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Expirer)),
                                input_names[0]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_latest() -> Self {
        Self::new(
            "Latest".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = make_input_getter!({}::new([Rc::clone(&{}), Rc::clone(&{})]), T, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Latest)),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new([Rc::clone(&{}), Rc::clone(&{})]));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Latest)),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new([{}.clone(), {}.clone()]));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Latest)),
                                input_names[0],
                                input_names[1]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_command_pid() -> Self {
        Self::new(
            "CommandPID".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!("let {} = panic!(\"streams::control::CommandPID available in RRTK 0.4+\");\n", var_name)
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::CommandPID
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::CommandPID
                                ))),
                                input_names[0],
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_ewma_stream() -> Self {
        Self::new(
            "EWMAStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = make_input_getter!({}::new(Rc::clone(&{}), todo!()), f32, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_moving_average_stream() -> Self {
        Self::new(
            "MovingAverageStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = make_input_getter!({}::new(Rc::clone(&{}), todo!()), f32, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::MovingAverageStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::MovingAverageStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::MovingAverageStream
                                ))),
                                input_names[0],
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_pid_controller_stream() -> Self {
        Self::new(
            "PIDControllerStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = make_input_getter!({}::new(Rc::clone(&{}), todo!(), todo!()), f32, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::PIDControllerStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::PIDControllerStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::PIDControllerStream
                                ))),
                                input_names[0],
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_position_to_state() -> Self {
        Self::new(
            "PositionToState".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::PositionToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::PositionToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::PositionToState
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_velocity_to_state() -> Self {
        Self::new(
            "VelocityToState".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::VelocityToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::VelocityToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::VelocityToState
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_acceleration_to_state() -> Self {
        Self::new(
            "AccelerationToState".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::AccelerationToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::AccelerationToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::AccelerationToState
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_none_to_error() -> Self {
        Self::new(
            "NoneToError".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{})), T, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToError
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToError
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToError
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_none_to_value() -> Self {
        Self::new(
            "NoneToValue".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{}), todo!(), todo!()), T, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToValue
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{}), todo!(), todo!()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToValue
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone(), todo!(), todo!()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToValue
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_float_to_quantity() -> Self {
        Self::new(
            "FloatToQuantity".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 | TargetVersion::V0_4 | TargetVersion::V0_5 => {
                            format!(
                                "let {} = panic!(\"FloatToQuantity available in RRTK 0.6+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new(todo!(), {}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::FloatToQuantity
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_quantity_to_float() -> Self {
        Self::new(
            "QuantityToFloat".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 | TargetVersion::V0_4 | TargetVersion::V0_5 => {
                            format!(
                                "let {} = panic!(\"QuantityToFloat available in RRTK 0.6+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::QuantityToFloat
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_freeze_stream() -> Self {
        Self::new(
            "FreezeStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"FreezeStream available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Flow(
                                    path::streams::Flow::FreezeStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Flow(
                                    path::streams::Flow::FreezeStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_if_stream() -> Self {
        Self::new(
            "IfStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"IfStream available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Flow(
                                    path::streams::Flow::IfStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Flow(
                                    path::streams::Flow::IfStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_if_else_stream() -> Self {
        Self::new(
            "IfElseStream".into(),
            3,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"IfElseStream available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{}), Rc::clone(&{})));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Flow(
                                    path::streams::Flow::IfElseStream
                                ))),
                                input_names[0],
                                input_names[1],
                                input_names[2]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), {}.clone(), {}.clone()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Flow(
                                    path::streams::Flow::IfElseStream
                                ))),
                                input_names[0],
                                input_names[1],
                                input_names[2]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_and_stream() -> Self {
        Self::new(
            "AndStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"AndStream available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Logic(
                                    path::streams::Logic::AndStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Logic(
                                    path::streams::Logic::AndStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_or_stream() -> Self {
        Self::new(
            "OrStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"OrStream available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Logic(
                                    path::streams::Logic::OrStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Logic(
                                    path::streams::Logic::OrStream
                                ))),
                                input_names[0],
                                input_names[1]
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_not_stream() -> Self {
        Self::new(
            "NotStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = panic!(\"NotStream available in RRTK 0.4+\");\n",
                                var_name
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Logic(
                                    path::streams::Logic::NotStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}.clone()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Logic(
                                    path::streams::Logic::NotStream
                                ))),
                                input_names[0],
                            )
                        }
                    }
                },
            ),
        )
    }
    pub fn new_sum_stream() -> Self {
        Self::new(
            "SumStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new([Rc::clone(&{}), Rc::clone(&{})]), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::SumStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new([Rc::clone(&{}), Rc::clone(&{})]));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::SumStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new([{}.clone(), {}.clone()]));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::SumStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_sum_2() -> Self {
        Self::new(
            "Sum2".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 | TargetVersion::V0_4 | TargetVersion::V0_5 => {
                            format!("let {} = panic!(\"Sum2 available in RRTK 0.6+\")", var_name,)
                        }
                        TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::Sum2
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_difference_stream() -> Self {
        Self::new(
            "DifferenceStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{}), Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::DifferenceStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::DifferenceStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::DifferenceStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_product_stream() -> Self {
        Self::new(
            "ProductStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new([Rc::clone(&{}), Rc::clone(&{})]), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::ProductStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new([Rc::clone(&{}), Rc::clone(&{})]));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::ProductStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new([{}.clone(), {}.clone()]));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::ProductStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_product_2() -> Self {
        Self::new(
            "Product2".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 | TargetVersion::V0_4 | TargetVersion::V0_5 => {
                            format!(
                                "let {} = panic!(\"Product2 available in RRTK 0.6+\")",
                                var_name,
                            )
                        }
                        TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::Product2
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_quotient_stream() -> Self {
        Self::new(
            "QuotientStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{}), Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::QuotientStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::QuotientStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::QuotientStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_exponent_stream() -> Self {
        Self::new(
            "ExponentStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{}), Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::ExponentStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{}), Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::ExponentStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone(), {}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::ExponentStream
                            ))),
                            input_names[0],
                            input_names[1],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_derivative_stream() -> Self {
        Self::new(
            "DerivativeStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::DerivativeStream
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::DerivativeStream
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::DerivativeStream
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
    pub fn new_integral_stream() -> Self {
        Self::new(
            "IntegralStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!({}::new(Rc::clone(&{})), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::IntegralStream
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new(Rc::clone(&{})));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::IntegralStream
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}.clone()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Math(
                                path::streams::Math::IntegralStream
                            ))),
                            input_names[0],
                        ),
                    }
                },
            ),
        )
    }
}
