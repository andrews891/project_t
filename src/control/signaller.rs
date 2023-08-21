use log::debug;
use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::graphmap::DiGraphMap;
use crate::infrastructure::train;
use crate::{
    infrastructure::{
        signal::{Owner, SignalColour}, block::{Block, BlockType}
    },
    control::{
        message::{SignallerMessage, TrainMessage},
    },
    utils::{
        bihashmap::BiHashMap
    }
};
use rayon::prelude::*;
use std::sync::mpsc::channel;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver}};


#[derive(Debug)]
pub struct Signaller <'a> {
    tx: HashMap<&'a str, Sender<SignallerMessage<'a>>>, 
    rx: Receiver<TrainMessage<'a>>,
    pub network: DiGraphMap::<&'a str, Arc<Mutex<Block<'a>>>>, // track id and its SUBSEQUENT tracks
    pub train_positions: BiHashMap<&'a str, &'a str>
}

// signaller controls everything (even trains, which relay information after every update)
// OWNERSHIP:
//
// # signaller 
// - trains (mut)
// - signals + tracks between each (mut)

impl <'a> Signaller <'a>{
    pub fn new(rx: Receiver<TrainMessage<'a>>, network: DiGraphMap<&'a str, Arc<Mutex<Block<'a>>>>) -> Self {
        Signaller {
            tx: HashMap::<&'a str, Sender<SignallerMessage<'a>>>::new(),
            rx,
            network,
            train_positions: BiHashMap::new()
        }
    }

    pub fn update(&mut self) {
        loop { // process incoming messages from previous update step
            match self.rx.try_recv() {
                Ok(message) => {
                    match message {
                        TrainMessage::HelloWorld { tx, train_id, block_id } => {
                            self.tx.insert(train_id, tx);
                            let prev_block_id = self.prev_in_path(train_id, block_id);
                            self.train_positions.insert(&block_id, &train_id);
                            self.reserve_block(prev_block_id, block_id, train_id);
                        },
                        TrainMessage::ReserveNextBlock { train_id } => {
                            let block_id = *self.train_positions.get(&"", &train_id).1.unwrap();
                            let next_block_id= self.next_in_path(train_id, block_id);
                            self.train_positions.remove(&"", &train_id);
                            self.train_positions.insert(&next_block_id, &train_id);
                            self.reserve_block(block_id, next_block_id, train_id);
                        }
                    }
                },
                Err(_) => {
                    break
                },
            }
        };
    }

    fn reserve_block(&self, block_id: &'a str, next_block_id: &'a str, train_id: &'a str) {
        debug!("reserving block {} for {}", next_block_id, train_id);
        let next_block = &self.network.edge_weight(block_id, next_block_id).unwrap().lock().unwrap();
        
        match &next_block.block_type {
            BlockType::Track { signal } => {
                self.tx.get(train_id).unwrap().send(SignallerMessage::NewBlock { 
                    new_block_id: next_block_id, 
                    colour: signal.colour,
                    limit: next_block.limit, 
                    length: next_block.length,
                }).unwrap();
            },
            BlockType::Station { platforms: _ } => (),
        }

        self.propagate_signal(block_id, Owner::Train { id: train_id }, SignalColour::Red );
    }

    fn next_in_path(&self, _train_id: &'a str, block_id: &'a str) -> &'a str { // signature needs changing back to train_id: &str only
        return self.network.edges_directed(block_id, Outgoing).next().unwrap().1 // 1 because outgoing
    }

    fn prev_in_path(&self, _train_id: &'a str, block_id: &'a str) -> &'a str { // signature needs changing back to train_id: &str only
        return self.network.edges_directed(block_id,Incoming).next().unwrap().0 // 0 because incoming
    }

    pub fn propagate_signal(&self, block_id: &'a str, owner: Owner<'a>, colour: SignalColour) {
        let prev_block_ids = self.network.neighbors_directed(block_id, Incoming);
        
        for prev_block_id in prev_block_ids {
            let mut prev_block = self.network.edge_weight(prev_block_id, block_id).unwrap().lock().unwrap();
            match &mut prev_block.block_type {
                BlockType::Track { signal } => {
                    if signal.update(owner, colour) {
                        let next_colour = colour.next();

                        match self.train_positions.get(&prev_block_id, &"").0 {
                            Some(train_id) => self.tx.get(train_id).unwrap().send(SignallerMessage::UpdateBlock { colour: next_colour, limit: prev_block.limit }).unwrap(),
                            None => (),
                        };

                        self.propagate_signal(prev_block_id, owner, next_colour);
                    }
                },
                BlockType::Station { platforms: _ } => {
                    todo!();
                },
            }
        }
    }
}
