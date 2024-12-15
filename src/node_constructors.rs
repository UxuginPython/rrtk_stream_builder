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
                                "let {} = make_input_getter({}::new({}, todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Expirer)),
                                input_names[0]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}, todo!(), todo!()));\n",
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
                                "let {} = make_input_getter!({}::new([{}, {}]), T, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Latest)),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new([{}, {}]));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Latest)),
                                input_names[0],
                                input_names[1]
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new([{}, {}]));\n",
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
                                "let {} = make_input_getter({}::new({}, todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::CommandPID
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}, todo!(), todo!()));\n",
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
                                "let {} = make_input_getter!({}::new({}, todo!()), f32, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new({}, todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}, todo!()));\n",
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
                                "let {} = make_input_getter!({}::new({}, todo!()), f32, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new({}, todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::EWMAStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}, todo!()));\n",
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
    pub fn new_pid_controller_stream() -> Self {
        Self::new(
            "PIDControllerStream".into(),
            1,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => {
                            format!(
                                "let {} = make_input_getter!({}::new({}, todo!(), todo!()), f32, E);\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::PIDControllerStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_4 => {
                            format!(
                                "let {} = make_input_getter({}::new({}, todo!(), todo!()));\n",
                                var_name,
                                scope.string_path(path::Crate::Streams(path::Streams::Control(
                                    path::streams::Control::PIDControllerStream
                                ))),
                                input_names[0],
                            )
                        }
                        TargetVersion::V0_5 | TargetVersion::V0_6 => {
                            format!(
                                "let {} = static_reference!({}::new({}, todo!(), todo!()));\n",
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
                            "let {} = make_input_getter!({}::new({}), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::PositionToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new({}));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::PositionToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}));\n",
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
                            "let {} = make_input_getter!({}::new({}), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::VelocityToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new({}));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::VelocityToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}));\n",
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
                            "let {} = make_input_getter!({}::new({}), f32, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::AccelerationToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new({}));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::AccelerationToState
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}));\n",
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
                            "let {} = make_input_getter!({}::new({}), T, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToError
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new({}));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToError
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}));\n",
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
                            "let {} = make_input_getter!({}::new({}, todo!(), todo!()), T, E);\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToValue
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter({}::new({}, todo!(), todo!()));\n",
                            var_name,
                            scope.string_path(path::Crate::Streams(path::Streams::Converters(
                                path::streams::Converters::NoneToValue
                            ))),
                            input_names[0],
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!({}::new({}, todo!(), todo!()));\n",
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
                            "let {} = static_reference!({}::new(todo!(), {}));\n",
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
                            "let {} = static_reference!({}::new({}));\n",
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
    pub fn new_quotient_stream() -> Self {
        Self::new(
            "QuotientStream".into(),
            2,
            Box::new(
                |target_version, scope: &scope::Crate, var_name, input_names: Vec<String>| {
                    match target_version {
                        TargetVersion::V0_3 => format!(
                            "let {} = make_input_getter!(QuotientStream::new({}, {}), f32, E);\n",
                            var_name, input_names[0], input_names[1]
                        ),
                        TargetVersion::V0_4 => format!(
                            "let {} = make_input_getter(QuotientStream::new({}, {}));\n",
                            var_name, input_names[0], input_names[1]
                        ),
                        TargetVersion::V0_5 | TargetVersion::V0_6 => format!(
                            "let {} = static_reference!(QuotientStream::new({}, {}));\n",
                            var_name, input_names[0], input_names[1]
                        ),
                    }
                },
            ),
        )
    }
}
