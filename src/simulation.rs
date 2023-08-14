use crate::{train::*, signaller::*, signal::*, conversion::*, block::*};
use petgraph::prelude::DiGraphMap;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fmt::Debug, sync::{Arc, Mutex}};


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
            signaller: Signaller::new(init_network(), init_trains())
        }
    }

    pub fn time_step(&mut self, delta_time: f32, clock_time: f32) {
        self.signaller.update(delta_time, clock_time);
    }
}

fn init_network<'a>() -> DiGraphMap::<&'a str, Arc<Mutex<Block<'a>>>> {
    let mut network = DiGraphMap::<&str, Arc<Mutex<Block>>>::new();

    network.add_edge("F", "A", Arc::new(Mutex::new(Block::new_track(4000, 125, Signal::new()))));
    network.add_edge("A", "B", Arc::new(Mutex::new(Block::new_track(4000, 125, Signal::new()))));
    network.add_edge("B", "C", Arc::new(Mutex::new(Block::new_track(4000, 60, Signal::new()))));
    network.add_edge("C", "D", Arc::new(Mutex::new(Block::new_track(4000, 60, Signal::new()))));
    //network.add_edge("C", "D", Arc::new(Mutex::new(Block::new_station(4000, 30, vec![Platform::new(Signal::new(), 1000)]))));
    network.add_edge("D", "E", Arc::new(Mutex::new(Block::new_track(4000, 125, Signal::new()))));
    network.add_edge("E", "F", Arc::new(Mutex::new(Block::new_track(4000, 125, Signal::new()))));

    return network;
}

fn init_trains<'a>() -> Vec<(Train<'a>, &'a str, &'a str, &'a str, Vec<(&'a str, usize, u32)>)> {
    let mut trains = Vec::<(Train, &str, &str, &str, Vec<(&'a str, usize, u32)>)>::new();

    trains.push((class802!("802"), "A", "B", "802", vec![("D", 1, 2000)]));
    
    return trains;
}