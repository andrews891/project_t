use std::{hash::{Hash, Hasher}};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum SignalColour {
    Red,
    Yellow,
    DoubleYellow,
    Green
}

impl SignalColour {
    pub fn next(&self) -> Self {
        use SignalColour::{DoubleYellow, Green, Red, Yellow};
        match *self {
            Red => Yellow,
            Yellow => DoubleYellow,
            DoubleYellow => Green,
            Green => Green,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Owner <'a> {
    Signaller,
    Train {id: &'a str}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct Signal<'a> {
    pub colour: SignalColour,
    pub owner: Owner<'a>,
}

impl<'a> Signal <'a> {
    pub fn new() -> Self {
        Signal {
            colour: SignalColour::Green,
            owner: Owner::Signaller,
        }
    }

    pub fn update(&mut self, owner: Owner <'a>, colour: SignalColour) -> bool {
        match self.owner {
            Owner::Signaller => {
                match owner {
                    Owner::Signaller => {
                        self.colour = colour;
                        colour != SignalColour::Green
                    },
                    Owner::Train { id: _ } => {
                        if self.colour > colour {
                            self.owner = owner;
                            self.colour = colour;
                            colour != SignalColour::Green
                        }
                        else {
                            false
                        }
                    },
                }
            },
            Owner::Train { id: self_id } => {
                match owner {
                    Owner::Signaller => {
                        if self.colour >= colour {
                            self.owner = owner;
                            self.colour = colour;
                            colour != SignalColour::Green
                        }
                        else {
                            false
                        }
                    },
                    Owner::Train { id: other_id } => {
                        if self_id == other_id {
                            self.colour = colour;
                            colour != SignalColour::Green
                        }
                        else if self.colour > colour {
                            self.owner = owner;
                            self.colour = colour;
                            colour != SignalColour::Green
                        }
                        else {
                            false
                        }
                    },
                }
            },
        }
    }
}