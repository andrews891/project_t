use crate::{
    infrastructure::{
        signal::Signal, block::Block, train::*
    },
    control::{
        driver::Driver, signaller::Signaller, message::*,
    }, utils::visualiser::Visualiser
};
use petgraph::prelude::DiGraphMap;
use rayon::prelude::*;
use std::{fmt::Debug, sync::{Arc, Mutex}, time::{Duration, self}};
use std::thread;
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver, SyncSender};

use log::{info};

const BUF_SIZE: usize = 10;

#[derive(Clone, Debug, PartialEq)]
struct YamlTrack {
    name: String,
    length: f32,
    limit: f32,
    reverse: String,
    next_tracks: Vec<String>
}

pub struct Simulation <'a> {
    duration: f32,
    delta_time: f32,
    ticks_per_update: u32,
    speedup: f32,
    visualiser: Visualiser,
    signaller: Signaller <'a>,
    drivers: Vec<Driver<'a>>
}

impl <'a> Simulation <'a> {
    pub fn new(duration: f32, delta_time: f32, ticks_per_update: u32, speedup: f32) -> Self {
        let (train_tx, signaller_rx) = sync_channel::<TrainMessage>(BUF_SIZE);

        return Simulation {
            duration,
            delta_time,
            ticks_per_update: ticks_per_update + 1,
            speedup,
            visualiser: Visualiser::new(),
            signaller: Signaller::new(signaller_rx, init_network()),
            drivers: init_drivers(train_tx, delta_time)
        }
    }

    pub fn run(mut self) {
        let mut time_elapsed = 0.0;

        let mut ticks = 0;

        let sleeper = spin_sleep::SpinSleeper::default();

        let (tx, rx) = channel();

        let timer = std::thread::spawn(move || {
            loop {
                sleeper.sleep_s((self.delta_time / self.speedup) as f64);
                tx.send(()).expect("Timer Error");
            }
        });

        info!("started timer with interval {}", self.delta_time / self.speedup);
        
        while time_elapsed < self.duration {
            ticks += 1;

            self.time_step();

            if ticks == self.ticks_per_update {
                if !cfg!(feature = "logging") {
                    self.visualiser.update(time_elapsed, &self.drivers, &self.signaller.network);
                }
                ticks = 1;
            }
            
            time_elapsed += &self.delta_time;

            rx.recv().unwrap();
        }

        drop(timer);
    }

    fn time_step(&mut self) {
        self.signaller.update();
        
        self.drivers.par_iter_mut().for_each(|driver| {
            driver.time_step();
        });
    }
}

fn init_network<'a>() -> DiGraphMap::<&'a str, Arc<Mutex<Block<'a>>>> {
    let mut network = DiGraphMap::<&str, Arc<Mutex<Block>>>::new();

    network.add_edge("F", "A", Arc::new(Mutex::new(Block::new_track(4000, 125.0, Signal::new()))));
    network.add_edge("A", "B", Arc::new(Mutex::new(Block::new_track(4000, 125.0, Signal::new()))));
    network.add_edge("B", "C", Arc::new(Mutex::new(Block::new_track(4000, 60.0, Signal::new()))));
    network.add_edge("C", "D", Arc::new(Mutex::new(Block::new_track(4000, 60.0, Signal::new()))));
    //network.add_edge("C", "D", Arc::new(Mutex::new(Block::new_station(4000, 30, vec![Platform::new(Signal::new(), 1000)]))));
    network.add_edge("D", "E", Arc::new(Mutex::new(Block::new_track(4000, 125.0, Signal::new()))));
    network.add_edge("E", "F", Arc::new(Mutex::new(Block::new_track(4000, 125.0, Signal::new()))));

    network
}

fn init_drivers<'a>(tx: SyncSender<TrainMessage<'a>>, delta_time: f32) -> Vec<Driver<'a>> {
    let mut drivers = Vec::new();
    
    drivers.push(Driver::new(tx.clone(), class802!("802208"), "A", delta_time, vec![("D", 1, 2000)]));
    info!("added train to network: {}", "802208");
    drivers.push(Driver::new(tx.clone(), class802!("802212"), "C", delta_time, vec![("E", 1, 2000)]));
    info!("added train to network: {}", "802212");

    return drivers;
}