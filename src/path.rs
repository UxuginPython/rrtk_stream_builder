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
impl From<Crate> for rrtk_rsb::NodeType {
    fn from(was: Crate) -> Self {
        match was {
            Crate::ConstantGetter => rrtk_rsb::NodeType::ConstantGetter,
            Crate::NoneGetter => rrtk_rsb::NodeType::NoneGetter,
            Crate::Streams(inner) => inner.into(),
        }
    }
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
impl From<Streams> for rrtk_rsb::NodeType {
    fn from(was: Streams) -> Self {
        match was {
            Streams::Expirer => rrtk_rsb::NodeType::Expirer,
            Streams::Latest => rrtk_rsb::NodeType::Latest,
            Streams::Control(inner) => inner.into(),
            Streams::Converters(inner) => inner.into(),
            Streams::Flow(inner) => inner.into(),
            Streams::Logic(inner) => inner.into(),
            Streams::Math(inner) => inner.into(),
        }
    }
}
pub mod streams {
    #[derive(Clone, Copy)]
    pub enum Control {
        CommandPID,
        EWMAStream,
        MovingAverageStream,
        PIDControllerStream,
    }
    impl From<Control> for rrtk_rsb::NodeType {
        fn from(was: Control) -> Self {
            match was {
                Control::CommandPID => rrtk_rsb::NodeType::CommandPID,
                Control::EWMAStream => rrtk_rsb::NodeType::EWMAStream,
                Control::MovingAverageStream => rrtk_rsb::NodeType::MovingAverageStream,
                Control::PIDControllerStream => rrtk_rsb::NodeType::PIDControllerStream,
            }
        }
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
    impl From<Converters> for rrtk_rsb::NodeType {
        fn from(was: Converters) -> Self {
            match was {
                Converters::PositionToState => rrtk_rsb::NodeType::PositionToState,
                Converters::VelocityToState => rrtk_rsb::NodeType::VelocityToState,
                Converters::AccelerationToState => rrtk_rsb::NodeType::AccelerationToState,
                Converters::NoneToError => rrtk_rsb::NodeType::NoneToError,
                Converters::NoneToValue => rrtk_rsb::NodeType::NoneToValue,
                Converters::FloatToQuantity => rrtk_rsb::NodeType::FloatToQuantity,
                Converters::QuantityToFloat => rrtk_rsb::NodeType::QuantityToFloat,
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Flow {
        FreezeStream,
        IfStream,
        IfElseStream,
    }
    impl From<Flow> for rrtk_rsb::NodeType {
        fn from(was: Flow) -> Self {
            match was {
                Flow::FreezeStream => rrtk_rsb::NodeType::FreezeStream,
                Flow::IfStream => rrtk_rsb::NodeType::IfStream,
                Flow::IfElseStream => rrtk_rsb::NodeType::IfElseStream,
            }
        }
    }
    #[derive(Clone, Copy)]
    pub enum Logic {
        AndStream,
        OrStream,
        NotStream,
    }
    impl From<Logic> for rrtk_rsb::NodeType {
        fn from(was: Logic) -> Self {
            match was {
                Logic::AndStream => rrtk_rsb::NodeType::AndStream,
                Logic::OrStream => rrtk_rsb::NodeType::OrStream,
                Logic::NotStream => rrtk_rsb::NodeType::NotStream,
            }
        }
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
    impl From<Math> for rrtk_rsb::NodeType {
        fn from(was: Math) -> Self {
            match was {
                Math::SumStream => rrtk_rsb::NodeType::SumStream,
                Math::Sum2 => rrtk_rsb::NodeType::Sum2,
                Math::DifferenceStream => rrtk_rsb::NodeType::DifferenceStream,
                Math::ProductStream => rrtk_rsb::NodeType::ProductStream,
                Math::Product2 => rrtk_rsb::NodeType::Product2,
                Math::QuotientStream => rrtk_rsb::NodeType::QuotientStream,
                Math::ExponentStream => rrtk_rsb::NodeType::ExponentStream,
                Math::DerivativeStream => rrtk_rsb::NodeType::DerivativeStream,
                Math::IntegralStream => rrtk_rsb::NodeType::IntegralStream,
            }
        }
    }
}
