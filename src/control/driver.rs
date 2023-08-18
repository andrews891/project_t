use std::fmt::Display;

use tokio::sync::{mpsc, broadcast};

use crate::{
    infrastructure::{
        signal::SignalColour, train::Train
    },
    control::message::{SignallerMessage, TrainMessage}
};

#[derive(Debug)]
pub struct Driver <'a> {
    tx: mpsc::Sender<TrainMessage<'a>>,
    rx: broadcast::Receiver<SignallerMessage<'a>>,
    pub train: Train<'a>,
    src: &'a str,
    dst: &'a str,
    timetable: Vec<(&'a str, usize, u32)>,
}

impl <'a> Driver <'a> {
    pub fn new(tx: mpsc::Sender<TrainMessage<'a>>, rx: broadcast::Receiver<SignallerMessage<'a>>, train: Train<'a>, src: &'a str, timetable: Vec<(&'a str, usize, u32)>) -> Self {
        let driver = Driver {
            tx,
            rx,
            train,
            src,
            dst: "",
            timetable,
        };

        driver.tx.blocking_send(TrainMessage::HelloWorld { train_id: driver.train.name, block_id: src }).unwrap();

        driver
    }

    pub fn status(&self) -> (&'a str, &'a str) {
        (self.train.name, self.dst)
    }

    pub fn time_step(&mut self, delta_time: f32) {
        loop { // process incoming messages from previous update step
            match self.rx.try_recv() {
                Ok(message) => {
                    match message {
                        SignallerMessage::UpdateBlock { block_id, colour, limit, length} => {
                            self.train.block_length = length as f32;
                            self.train.position -= self.train.block_length;
                            self.src = self.dst;
                            self.dst = block_id; // take the first option for now

                            match colour {
                                SignalColour::Red => {
                                    self.train.target_velocity = 0.0;
                                },
                                SignalColour::Yellow => {
                                    self.train.target_velocity = 0.5 * (limit as f32);
                                },
                                SignalColour::DoubleYellow => {
                                    self.train.target_velocity = 0.75 * (limit as f32);
                                },
                                SignalColour::Green => {
                                    self.train.target_velocity = limit as f32;
                                }, 
                            }
                        },
                    }
                },
                Err(_) => {
                    break
                },
            }        
        };
                    
        self.train.target_distance = self.train.block_length - self.train.position;

        // update train position and set signals
        if self.train.position > self.train.block_length {
            //todo: train pathfinding - handle with signaller tho
            self.tx.blocking_send(TrainMessage::ReserveNextBlock { train_id: self.train.name }).unwrap();
        }
                
        // update train
        self.train.update(delta_time);    

    }
}

impl <'a> Display for Driver <'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>8} | {} |", self.train, self.train.name)
    }
}