// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use super::*;
impl Node {
    pub fn new_from_path(path: path::Crate) -> Self {
        match path {
            path::Crate::ConstantGetter => Self::new_constant_getter(),
            path::Crate::NoneGetter => Self::new_none_getter(),
            path::Crate::Streams(path::Streams::Expirer) => Self::new_expirer(),
            path::Crate::Streams(path::Streams::Latest) => Self::new_latest(),
            path::Crate::Streams(path::Streams::Control(path::streams::Control::CommandPID)) => {
                Self::new_command_pid()
            }
            path::Crate::Streams(path::Streams::Control(path::streams::Control::EWMAStream)) => {
                Self::new_ewma_stream()
            }
            path::Crate::Streams(path::Streams::Control(
                path::streams::Control::MovingAverageStream,
            )) => Self::new_moving_average_stream(),
            path::Crate::Streams(path::Streams::Control(
                path::streams::Control::PIDControllerStream,
            )) => Self::new_pid_controller_stream(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::PositionToState,
            )) => Self::new_position_to_state(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::VelocityToState,
            )) => Self::new_velocity_to_state(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::AccelerationToState,
            )) => Self::new_acceleration_to_state(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::NoneToError,
            )) => Self::new_none_to_error(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::NoneToValue,
            )) => Self::new_none_to_value(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::FloatToQuantity,
            )) => Self::new_float_to_quantity(),
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::QuantityToFloat,
            )) => Self::new_quantity_to_float(),
            path::Crate::Streams(path::Streams::Flow(path::streams::Flow::FreezeStream)) => {
                Self::new_freeze_stream()
            }
            path::Crate::Streams(path::Streams::Flow(path::streams::Flow::IfStream)) => {
                Self::new_if_stream()
            }
            path::Crate::Streams(path::Streams::Flow(path::streams::Flow::IfElseStream)) => {
                Self::new_if_else_stream()
            }
            path::Crate::Streams(path::Streams::Logic(path::streams::Logic::AndStream)) => {
                Self::new_and_stream()
            }
            path::Crate::Streams(path::Streams::Logic(path::streams::Logic::OrStream)) => {
                Self::new_or_stream()
            }
            path::Crate::Streams(path::Streams::Logic(path::streams::Logic::NotStream)) => {
                Self::new_not_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::SumStream)) => {
                Self::new_sum_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::Sum2)) => {
                Self::new_sum_2()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::DifferenceStream)) => {
                Self::new_difference_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::ProductStream)) => {
                Self::new_product_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::Product2)) => {
                Self::new_product_2()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::QuotientStream)) => {
                Self::new_quotient_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::ExponentStream)) => {
                Self::new_exponent_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::DerivativeStream)) => {
                Self::new_derivative_stream()
            }
            path::Crate::Streams(path::Streams::Math(path::streams::Math::IntegralStream)) => {
                Self::new_integral_stream()
            }
        }
    }
    pub fn new_from_rsb_type(id: rrtk_rsb::NodeType) -> Self {
        match id {
            rrtk_rsb::NodeType::ConstantGetter => Self::new_constant_getter(),
            rrtk_rsb::NodeType::NoneGetter => Self::new_none_getter(),
            rrtk_rsb::NodeType::Expirer => Self::new_expirer(),
            rrtk_rsb::NodeType::Latest => Self::new_latest(),
            rrtk_rsb::NodeType::CommandPID => Self::new_command_pid(),
            rrtk_rsb::NodeType::EWMAStream => Self::new_ewma_stream(),
            rrtk_rsb::NodeType::MovingAverageStream => Self::new_moving_average_stream(),
            rrtk_rsb::NodeType::PIDControllerStream => Self::new_pid_controller_stream(),
            rrtk_rsb::NodeType::PositionToState => Self::new_position_to_state(),
            rrtk_rsb::NodeType::VelocityToState => Self::new_velocity_to_state(),
            rrtk_rsb::NodeType::AccelerationToState => Self::new_acceleration_to_state(),
            rrtk_rsb::NodeType::NoneToError => Self::new_none_to_error(),
            rrtk_rsb::NodeType::NoneToValue => Self::new_none_to_value(),
            rrtk_rsb::NodeType::FloatToQuantity => Self::new_float_to_quantity(),
            rrtk_rsb::NodeType::QuantityToFloat => Self::new_quantity_to_float(),
            rrtk_rsb::NodeType::FreezeStream => Self::new_freeze_stream(),
            rrtk_rsb::NodeType::IfStream => Self::new_if_stream(),
            rrtk_rsb::NodeType::IfElseStream => Self::new_if_else_stream(),
            rrtk_rsb::NodeType::AndStream => Self::new_and_stream(),
            rrtk_rsb::NodeType::OrStream => Self::new_or_stream(),
            rrtk_rsb::NodeType::NotStream => Self::new_not_stream(),
            rrtk_rsb::NodeType::SumStream => Self::new_sum_stream(),
            rrtk_rsb::NodeType::Sum2 => Self::new_sum_2(),
            rrtk_rsb::NodeType::DifferenceStream => Self::new_difference_stream(),
            rrtk_rsb::NodeType::ProductStream => Self::new_product_stream(),
            rrtk_rsb::NodeType::Product2 => Self::new_product_2(),
            rrtk_rsb::NodeType::QuotientStream => Self::new_quotient_stream(),
            rrtk_rsb::NodeType::ExponentStream => Self::new_exponent_stream(),
            rrtk_rsb::NodeType::DerivativeStream => Self::new_derivative_stream(),
            rrtk_rsb::NodeType::IntegralStream => Self::new_integral_stream(),
            _ => unimplemented!(),
        }
    }
    pub fn new_constant_getter() -> Self {
        Self::new(
            path::Crate::ConstantGetter,
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
            path::Crate::NoneGetter,
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
            path::Crate::Streams(path::Streams::Expirer),
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
            path::Crate::Streams(path::Streams::Latest),
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
            path::Crate::Streams(path::Streams::Control(path::streams::Control::CommandPID)),
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
            path::Crate::Streams(path::Streams::Control(path::streams::Control::EWMAStream)),
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
            path::Crate::Streams(path::Streams::Control(
                path::streams::Control::MovingAverageStream,
            )),
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
            path::Crate::Streams(path::Streams::Control(
                path::streams::Control::PIDControllerStream,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::PositionToState,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::VelocityToState,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::AccelerationToState,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::NoneToError,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::NoneToValue,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::FloatToQuantity,
            )),
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
            path::Crate::Streams(path::Streams::Converters(
                path::streams::Converters::QuantityToFloat,
            )),
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
            path::Crate::Streams(path::Streams::Flow(path::streams::Flow::FreezeStream)),
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
            path::Crate::Streams(path::Streams::Flow(path::streams::Flow::IfStream)),
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
            path::Crate::Streams(path::Streams::Flow(path::streams::Flow::IfElseStream)),
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
            path::Crate::Streams(path::Streams::Logic(path::streams::Logic::AndStream)),
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
            path::Crate::Streams(path::Streams::Logic(path::streams::Logic::OrStream)),
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
            path::Crate::Streams(path::Streams::Logic(path::streams::Logic::NotStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::SumStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::Sum2)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::DifferenceStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::ProductStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::Product2)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::QuotientStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::ExponentStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::DerivativeStream)),
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
            path::Crate::Streams(path::Streams::Math(path::streams::Math::IntegralStream)),
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
