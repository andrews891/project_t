use crate::infrastructure::signal::SignalColour;

#[derive(Debug, Clone)]
pub enum TrainMessage <'m> {
    HelloWorld { train_id: &'m str, block_id: &'m str },
    ReserveNextBlock { train_id: &'m str }
}

#[derive(Debug, Clone)]
pub enum SignallerMessage <'m> {
    UpdateBlock { block_id: &'m str, colour: SignalColour, limit: u32, length: u32 }
}