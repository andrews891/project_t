use crate::{
    infrastructure::{
        signal::Signal, platform::Platform
    },
    utils::{
        conversion::convert_to_mps
    }
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum BlockType <'a> {
    Track { signal: Signal<'a> },
    Station { platforms: Vec<Platform<'a>> }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Block <'a> {
    pub length: u32,
    pub limit: u32,
    pub block_type: BlockType<'a>,
}


impl <'a> Block <'a> {
    pub fn new_track(length: u32, limit: u32, signal: Signal<'a>,) -> Self {
        Block {
            length,
            limit: convert_to_mps(limit),
            block_type: BlockType::Track { 
                signal
            }
        }
    }

    pub fn new_station(length: u32, limit: u32, platforms: Vec<Platform<'a>>) -> Self {
        Block {
            length,
            limit: convert_to_mps(limit),
            block_type: BlockType::Station {
                platforms,
            }
        }
    }

    pub fn add_platform(&mut self, platform: Platform<'a>) {
        match &mut self.block_type {
            BlockType::Track { signal: _ } => {
                panic!("CANNOT ADD PLATFORM TO TRACK");
            },
            BlockType::Station { platforms } => {
                platforms.push(platform);
            },
        }
    }
}