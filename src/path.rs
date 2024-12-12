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
}
