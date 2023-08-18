use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::graphmap::DiGraphMap;
use tokio::sync::{broadcast, mpsc};
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

use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct Signaller <'a> {
    tx: broadcast::Sender<SignallerMessage<'a>>, 
    rx: mpsc::Receiver<TrainMessage<'a>>,
    pub network: DiGraphMap::<&'a str, Arc<Mutex<Block<'a>>>>, // track id and its SUBSEQUENT tracks
    train_positions: BiHashMap<&'a str, &'a str>
}

// signaller controls everything (even trains, which relay information after every update)
// OWNERSHIP:
//
// # signaller 
// - trains (mut)
// - signals + tracks between each (mut)

impl <'a> Signaller <'a>{
    pub fn new(tx: broadcast::Sender<SignallerMessage<'a>>, rx: mpsc::Receiver<TrainMessage<'a>>, network: DiGraphMap<&'a str, Arc<Mutex<Block<'a>>>>) -> Self {
        Signaller {
            tx,
            rx,
            network,
            train_positions: BiHashMap::new()
        }
    }

    pub async fn update(&mut self) {
        loop { // process incoming messages from previous update step
            match self.rx.try_recv() {
                Ok(message) => {
                    match message {
                        TrainMessage::HelloWorld { train_id, block_id } => {
                            let next_block_id= self.next_in_path(train_id, block_id);
                            self.propagate_signal(block_id, Owner::Train { id: train_id }, SignalColour::Red );
                            self.train_positions.insert(next_block_id, train_id);
                            self.reserve_block(block_id, next_block_id);
                        },
                        TrainMessage::ReserveNextBlock { train_id } => {
                            let block_id = *self.train_positions.get(&"", &train_id).1.unwrap();
                            let next_block_id= self.next_in_path(train_id, block_id);
                            self.propagate_signal(block_id, Owner::Train { id: train_id }, SignalColour::Red );
                            self.train_positions.remove(&"", &train_id);
                            self.train_positions.insert(next_block_id, train_id);
                            self.reserve_block(block_id, next_block_id);
                        }
                    }
                },
                Err(_) => {
                    break
                },
            }
        };
    }

    fn reserve_block(&self, block_id: &'a str, next_block_id: &'a str) {
        let block = &self.network.edge_weight(block_id, next_block_id).unwrap().lock().unwrap();
                        
        match &block.block_type {
            BlockType::Track { signal } => {
                self.tx.send(SignallerMessage::UpdateBlock { 
                    block_id, 
                    colour: signal.colour,
                    limit: block.limit, 
                    length: block.length,
                }).unwrap();
            },
            BlockType::Station { platforms: _ } => (),
        }
    }

    fn next_in_path(&self, _train_id: &'a str, block_id: &'a str) -> &'a str { // signature needs changing back to train_id: &str only
        return self.network.edges_directed(block_id, Outgoing).next().unwrap().1
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
                            Some(_) => self.tx.send(SignallerMessage::UpdateBlock { block_id: prev_block_id, colour: next_colour, limit: prev_block.limit, length: prev_block.length }).unwrap(),
                            None => 0,
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
