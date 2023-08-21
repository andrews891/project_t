use std::sync::mpsc::Sender;

use crate::infrastructure::signal::SignalColour;

#[derive(Debug, Clone)]
pub enum TrainMessage <'m> {
    HelloWorld { tx: Sender<SignallerMessage<'m>>, train_id: &'m str, block_id: &'m str },
    ReserveNextBlock { train_id: &'m str }
}

#[derive(Debug, Clone)]
pub enum SignallerMessage <'m> {
    NewBlock { new_block_id: &'m str, colour: SignalColour, limit: f32, length: u32 },
    UpdateBlock { colour: SignalColour, limit: f32 }
}