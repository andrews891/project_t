use console::{Term, Style};
use crate::{
    infrastructure::{
        signal::SignalColour, block::{Block, BlockType}
    },
    control::driver::Driver
};
use petgraph::graphmap::DiGraphMap;
use std::sync::{Arc, Mutex};

pub struct Visualiser {
    term: Term,
    r: Style,
    y: Style,
    dy: Style,
    g: Style,
}

impl Visualiser {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),

            r: Style::new().red(),
            y: Style::new().yellow(),
            dy: Style::new().color256(172),
            g: Style::new().green(),
        }
    }
    
    pub fn update(&self, time_elapsed: f32, drivers: &Vec::<Driver>, network: &DiGraphMap::<&str, Arc<Mutex<Block>>>) {
        self.term.clear_screen().unwrap();

        self.term.write_line(&format!("Time: {time_elapsed:>9.2}s")).unwrap();

        let mut train_locations = Vec::<(&str, &str)>::new();

        for driver in drivers {
            self.term.write_line(&format!("{driver}")).unwrap();
            train_locations.push(driver.status());
        }

        for block in network.all_edges() {
            let mut colour: &Style = &self.r;
            let mut train: &str = "";
            
            match &block.2.lock().unwrap().block_type {
                BlockType::Track { signal } => {
                    match signal.colour {
                        SignalColour::Red => colour = &self.r,
                        SignalColour::Yellow => colour = &self.y,
                        SignalColour::DoubleYellow => colour = &self.dy,
                        SignalColour::Green => colour = &self.g,
                    }
                },
                BlockType::Station { platforms: _ } => {
                    
                },
            }

            for loc in &train_locations {
                if loc.1 == block.1 {
                    train = loc.0;
                }
            }

            self.term.write_str(&format!("{:>7} {} ", train, colour.apply_to(block.1))).unwrap();
        }
        
    }
}
