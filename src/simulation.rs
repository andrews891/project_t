use crate::{train::*, signaller::*};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fmt::Debug};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct YamlTrack {
    name: String,
    length: f32,
    limit: f32,
    reverse: String,
    next_tracks: Vec<String>
}
#[derive(Debug)]
pub struct Simulation <'a> {
    pub signaller: Signaller <'a>
}

impl <'a> Simulation <'a> {
    pub fn new() -> Self {
        return Simulation {
            signaller: Signaller::new()
        }
    }

    pub fn time_step(&mut self, delta_time: f32) {
        self.signaller.update(delta_time, delta_time.rem_euclid(1.0) < 0.02);
    }
}
