// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
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
    Logic(streams::Logic),
    Math(streams::Math),
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
    #[derive(Clone, Copy)]
    pub enum Logic {
        AndStream,
        OrStream,
        NotStream,
    }
    #[derive(Clone, Copy)]
    pub enum Math {
        SumStream,
        Sum2,
        DifferenceStream,
        ProductStream,
        Product2,
        QuotientStream,
        ExponentStream,
        DerivativeStream,
        IntegralStream,
    }
}
