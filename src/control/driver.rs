use std::fmt::Display;

use std::sync::mpsc::{SyncSender, Receiver, channel};
use log::debug;

use crate::{
    infrastructure::{
        signal::SignalColour, train::Train
    },
    control::message::{SignallerMessage, TrainMessage}
};

#[derive(Debug)]
pub struct Driver <'a> {
    tx: SyncSender<TrainMessage<'a>>,
    rx: Receiver<SignallerMessage<'a>>,
    pub train: Train<'a>,
    src: &'a str,
    pub dst: &'a str,
    delta_time: f32,
    timetable: Vec<(&'a str, usize, u32)>,
}

impl <'a> Driver <'a> {
    pub fn new(tx: SyncSender<TrainMessage<'a>>, train: Train<'a>, dst: &'a str, delta_time: f32, timetable: Vec<(&'a str, usize, u32)>) -> Self {
        let (signaller_tx, rx) = channel();

        let driver = Driver {
            tx,
            rx,
            train,
            src: "",
            dst,
            delta_time,
            timetable,
        };

        
        driver.tx.send(TrainMessage::HelloWorld { tx: signaller_tx, train_id: driver.train.name, block_id: dst }).unwrap();

        driver
    }

    pub fn status(&self) -> (&'a str, &'a str) {
        (self.train.name, self.dst)
    }

    pub fn time_step(&mut self) {
        loop { // process incoming messages from previous update step
            match self.rx.try_recv() {
                Ok(message) => {
                    match message {
                        SignallerMessage::NewBlock { new_block_id, colour, limit, length } => {
                            self.train.position -= self.train.block_length; // subtract the previous block length from position to get ~0
                            self.train.block_length = length as f32; // update for new block length
                            self.src = self.dst;
                            self.dst = new_block_id;
                            
                            debug!("{} entered block {}", self.train.name, new_block_id);
                            self.adjust_speed(colour, limit as f32);
                        },
                        SignallerMessage::UpdateBlock { colour, limit} => {
                            debug!("{} received signal update", self.train.name);
                            self.adjust_speed(colour, limit as f32);
                        },
                    }
                },
                Err(_) => {            
                    break;
                },
            }        
        };

        self.train.target_distance = self.train.block_length - self.train.position;

        // update train position and set signals
        if self.train.position > self.train.block_length {
            //todo: train pathfinding - handle with signaller tho
            self.tx.send(TrainMessage::ReserveNextBlock { train_id: self.train.name }).unwrap();
            debug!("{} reserving next block", self.train.name);
        }

        // update train
        self.train.update(self.delta_time);
    }

    fn adjust_speed(&mut self, colour: SignalColour, limit: f32) {
        match colour {
            SignalColour::Red => {
                self.train.target_velocity = 0.0;
            },
            SignalColour::Yellow => {
                self.train.target_velocity = 0.5 * limit;
            },
            SignalColour::DoubleYellow => {
                self.train.target_velocity = 0.75 * limit;
            },
            SignalColour::Green => {
                self.train.target_velocity = limit;
            }, 
        }
        debug!("{} set target velocity to {}", self.train.name, self.train.target_velocity);
    }
}

impl <'a> Display for Driver <'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>8} | {} |", self.train, self.train.name)
    }
}