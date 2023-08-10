use std::{cmp::Ordering, hash::*};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum SignalColour {
    Red,
    DoubleYellow,
    Yellow,
    Green
}

impl SignalColour {
    pub fn next(&self) -> Self {
        use SignalColour::*;
        match *self {
            Red => DoubleYellow,
            DoubleYellow => Yellow,
            Yellow => Green,
            Green => Green,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Signal<'a> {
    pub block_length: u32,
    pub block_limit: u32,
    pub colour: SignalColour,
    pub train: Option<&'a str>,
}

impl<'a> Signal <'a> {
    pub fn new(block_length: u32, block_limit: u32) -> Self {
        return Signal {
            block_length: block_length,
            block_limit: block_limit,
            colour: SignalColour::Green,
            train: None,
        }
    }
}