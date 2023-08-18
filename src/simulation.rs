use crate::{
    infrastructure::{
        signal::Signal, block::Block, train::*
    },
    control::{
        driver::Driver, signaller::Signaller, message::*,
    }
};
use petgraph::prelude::DiGraphMap;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};
use std::{fmt::Debug, sync::{Arc, Mutex}};
use tokio::sync::{mpsc, broadcast};



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
    pub signaller: Signaller <'a>,
    pub drivers: Vec<Driver<'a>>
}

impl <'a> Simulation <'a> {
    pub fn new() -> Self {
        let (signaller_tx, train_rx) = broadcast::channel::<SignallerMessage>(100);
        let (train_tx, signaller_rx) = mpsc::channel::<TrainMessage>(100);

        return Simulation {
            signaller: Signaller::new(signaller_tx, signaller_rx, init_network()),
            drivers: init_drivers(train_tx, train_rx)
        }
    }

    pub async fn time_step(&mut self, delta_time: f32) {
        self.signaller.update().await;
        
        self.drivers.par_iter_mut().for_each(|driver| {
            driver.time_step(delta_time);
        });
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

    network
}

fn init_drivers<'a>(tx: mpsc::Sender<TrainMessage<'a>>, rx: broadcast::Receiver<SignallerMessage<'a>>) -> Vec<Driver<'a>> {
    let mut drivers = Vec::<Driver>::new();
    
    drivers.push(Driver::new(tx, rx, class802!("802"), "A", vec![("D", 1, 2000)]));

    return drivers;
}