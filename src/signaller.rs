use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::graphmap::DiGraphMap;
use crate::train::*;
use crate::signal::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use crate::conversion::*;

#[derive(Debug)]
pub struct Signaller <'a> {
    pub network: DiGraphMap::<&'a str, Arc<Mutex<Signal<'a>>>>, // track id and its SUBSEQUENT tracks
    pub trains: Vec<(Train, &'a str, &'a str, &'a str)>, // train + src id + dst id + distance along track + name
}

// signaller controls everything (even trains, which relay information after every update)
// OWNERSHIP:
//
// # signaller 
// - trains (mut)
// - signals + tracks between each (mut)

impl <'a> Signaller <'a>{
    pub fn new() -> Self {
        let signaller = Signaller {
            network: init_network(),
            trains: init_trains(),
        };

        signaller.trains.iter().for_each(|train| {
            signaller.propagate_signal(train.1, train.3, SignalColour::Red);
        });

        return signaller;
    }

    pub fn update(&mut self, delta_time: f32, control: bool) {
        let updated_signals = Arc::new(Mutex::new(Vec::<(&str, &str)>::new()));

        self.trains.par_iter_mut().for_each(|train| {
            // update speed limits based on signal states
            let src = train.1;
            let dst = train.2;
            
            let next_signal = self.network.edge_weight(src, dst).unwrap().lock().unwrap();
            
            match next_signal.colour {
                SignalColour::Red => {
                    train.0.target_velocity = 0.0;
                }
                SignalColour::DoubleYellow => {
                    train.0.target_velocity = 0.5 * (next_signal.block_limit as f32);
                },
                SignalColour::Yellow => {
                    train.0.target_velocity = 0.75 * (next_signal.block_limit as f32);
                },
                SignalColour::Green => {
                    train.0.target_velocity = next_signal.block_limit as f32;
                }, 
            }

            train.0.target_distance = next_signal.block_length as f32 - train.0.position;
            
            // update train
            train.0.update(delta_time, control);

            // update train position and set signals
            match train.0.position > next_signal.block_length as f32 {
                true => {
                    train.0.position -= next_signal.block_length as f32;
                    train.1 = dst;
                    train.2 = self.network.neighbors_directed(dst, Outgoing).next().unwrap(); // take the first option for now
                    //todo: train pathfinding
                    updated_signals.lock().unwrap().push((dst, train.3));
                }
                false => (),
            }
        });

        updated_signals.lock().unwrap().par_iter().for_each(|(signal, train)| {
            self.propagate_signal(signal, train, SignalColour::Red);
        });
    }

    pub fn propagate_signal(&self, signal_id: &str, train: &'a str, colour: SignalColour) {
        let prev_signal_ids = self.network.neighbors_directed(signal_id, Incoming);

        prev_signal_ids.into_iter().for_each(|prev_signal_id| {
            let mut prev_signal = self.network.edge_weight(prev_signal_id, signal_id).unwrap().lock().unwrap();
            
            match colour {
                _ => {
                    if prev_signal.colour < colour {
                        if prev_signal.train == None || prev_signal.train == Some(train) {
                            prev_signal.train = Some(train);
                            prev_signal.colour = colour;
                            self.propagate_signal(prev_signal_id, prev_signal.train.unwrap(), colour.next());
                        }
                    }
                    else { // claim the signal if train has bigger impact
                        dbg!();
                        prev_signal.train = Some(train);
                        prev_signal.colour = colour;
                        self.propagate_signal(prev_signal_id, prev_signal.train.unwrap(), colour.next());
                    }
                }
            }
        })
    }
}

fn init_network<'a>() -> DiGraphMap::<&'a str, Arc<Mutex<Signal<'a>>>> {
    let mut network = DiGraphMap::<&str, Arc<Mutex<Signal>>>::new();
    
    network.add_edge("A", "B", Arc::new(Mutex::new(Signal::new(2000, convert_to_mps(125.0)))));
    network.add_edge("B", "C", Arc::new(Mutex::new(Signal::new(3000, convert_to_mps(125.0)))));
    network.add_edge("C", "D", Arc::new(Mutex::new(Signal::new(2000, convert_to_mps(75.0)))));
    network.add_edge("D", "E", Arc::new(Mutex::new(Signal::new(2500, convert_to_mps(125.0)))));
    network.add_edge("E", "F", Arc::new(Mutex::new(Signal::new(2000, convert_to_mps(125.0)))));
    network.add_edge("F", "G", Arc::new(Mutex::new(Signal::new(3000, convert_to_mps(125.0)))));
    network.add_edge("G", "H", Arc::new(Mutex::new(Signal::new(2000, convert_to_mps(75.0)))));
    network.add_edge("H", "I", Arc::new(Mutex::new(Signal::new(4000, convert_to_mps(125.0)))));

    return network;
}

fn init_trains<'a>() -> Vec<(Train, &'a str, &'a str, &'a str)> {
    let mut trains = Vec::<(Train, &str, &str, &str)>::new();

    trains.push((class802!("802"), "A", "B", "802"));
    trains.push((class802!("802-2"), "C", "D", "802-2"));

    return trains;
}
