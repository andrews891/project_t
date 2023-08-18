use crate::infrastructure::signal::Signal;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Platform<'a> {
    pub signal: Signal<'a>,
    pub length: u32,
    pub occupant: Option<&'a str>
}

impl <'a> Platform<'a> {
    pub fn new(signal: Signal<'a>, length: u32) -> Self {
        Platform {
            signal,
            length,
            occupant: None,
        }
    }
}