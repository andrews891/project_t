use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::graphmap::DiGraphMap;
use crate::train::*;
use crate::signal::*;
use crate::block::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use crate::conversion::*;

#[derive(Debug)]
pub struct Signaller <'a> {
    pub network: DiGraphMap::<&'a str, Arc<Mutex<Block<'a>>>>, // track id and its SUBSEQUENT tracks
    pub trains: Vec<(Train<'a>, &'a str, &'a str, &'a str, Vec<(&'a str, usize, u32)>)>, // train + src id + dst id + distance along track + timetable
}

// signaller controls everything (even trains, which relay information after every update)
// OWNERSHIP:
//
// # signaller 
// - trains (mut)
// - signals + tracks between each (mut)

impl <'a> Signaller <'a>{
    pub fn new(network: DiGraphMap<&'a str, Arc<Mutex<Block<'a>>>>, trains: Vec<(Train<'a>, &'a str, &'a str, &'a str, Vec<(&'a str, usize, u32)>)>) -> Self {
        let signaller = Signaller {
            network: network,
            trains: trains,
        };

        signaller.trains.iter().for_each(|train| {
            signaller.propagate_signal(train.1, Owner::Train{ id: train.0.name }, SignalColour::Red);
        });

        return signaller;
    }

    pub fn update(&mut self, delta_time: f32, clock_time: f32) {
        let updated_signals = Arc::new(Mutex::new(Vec::<(&str, &str)>::new()));

        self.trains.par_iter_mut().for_each(|train| {
            // update speed limits based on signal states
            let src = train.1;
            let dst = train.2;
            
            let mut next_block = self.network.edge_weight(src, dst).unwrap().lock().unwrap();
            
            match &mut next_block.block_type {
                BlockType::Track { signal } => {
                    match signal.colour {
                        SignalColour::Red => {
                            train.0.target_velocity = 0.0;
                        }
                        SignalColour::DoubleYellow => {
                            train.0.target_velocity = 0.5 * (next_block.limit as f32);
                        },
                        SignalColour::Yellow => {
                            train.0.target_velocity = 0.75 * (next_block.limit as f32);
                        },
                        SignalColour::Green => {
                            train.0.target_velocity = next_block.limit as f32;
                        }, 
                    }
                },
                BlockType::Station { platforms,  } => {
                    todo!();
                },
            }

            train.0.target_distance = next_block.length as f32 - train.0.position;
            
            // update train
            train.0.update(delta_time);

            // update train position and set signals
            if train.0.position > next_block.length as f32 {
                train.0.position -= next_block.length as f32;
                train.1 = dst;
                train.2 = self.network.neighbors_directed(dst, Outgoing).next().unwrap(); // take the first option for now
                //todo: train pathfinding
                updated_signals.lock().unwrap().push((dst, train.3));
            }
        });

        updated_signals.lock().unwrap().par_iter().for_each(|(signal, train)| {
            self.propagate_signal(signal, Owner::Train { id: train }, SignalColour::Red); // may be able to ref directly with arc and mutex
        });
    }

    pub fn propagate_signal(&self, block_id: &str, owner: Owner<'a>, colour: SignalColour) {
        let prev_block_ids = self.network.neighbors_directed(block_id, Incoming);

        for prev_block_id in prev_block_ids {
            let mut prev_block = self.network.edge_weight(prev_block_id, block_id).unwrap().lock().unwrap();

            match &mut prev_block.block_type {
                BlockType::Track { signal } => {
                    if signal.update(owner, colour) {
                        self.propagate_signal(prev_block_id, owner, colour.next());
                    }
                },
                BlockType::Station { platforms } => {
                    todo!();
                },
            }
        }
    }
}
