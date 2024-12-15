// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
#![allow(unused)]
use super::*;
#[derive(Clone, Copy)]
pub enum Crate {
    ConstantGetter,
    NoneGetter,
    Streams(Streams),
}
#[derive(Clone, Copy)]
pub enum Streams {
    Expirer,
    Latest,
    Control(streams::Control),
    Converters(streams::Converters),
    Flow(streams::Flow),
}
pub mod streams {
    #[derive(Clone, Copy)]
    pub enum Control {
        CommandPID,
        EWMAStream,
        MovingAverageStream,
        PIDControllerStream,
    }
    #[derive(Clone, Copy)]
    pub enum Converters {
        PositionToState,
        VelocityToState,
        AccelerationToState,
        NoneToError,
        NoneToValue,
        FloatToQuantity,
        QuantityToFloat,
    }
    #[derive(Clone, Copy)]
    pub enum Flow {
        FreezeStream,
        IfStream,
        IfElseStream,
    }
}
