#[macro_use] mod train;
mod surface;
mod conversion;
mod simulation;
mod signaller;
mod signal;
mod io;
mod block;

use crate::train::*;
use crate::surface::*;
use crate::simulation::*;
use crate::signal::*;
use conversion::convert_to_mps;
use tokio::{task, time};
use crate::signal::SignalColour;
use console::{Term, Style};

const GRAVITY: f32 = 9.81;

#[tokio::main]
async fn main() {
    let term = Term::stdout();

    let r = Style::new().red();
    let dy = Style::new().color256(172);
    let y = Style::new().yellow();
    let g = Style::new().green();


    let mut simulation = Simulation::new();

    dbg!();

    let duration = 4000.0;
    let time_step = 0.01;
    let speedup = 400.0;

    let mut time_elapsed = 0.0;

    let mut ticks = (speedup - 1.0) as u32;

    let mut interval = time::interval(time::Duration::from_secs_f32(time_step / speedup));

    println!("Initialised");
    
    while time_elapsed < duration {
        ticks += 1;

        interval.tick().await;

        simulation.time_step(time_step, time_elapsed);

        if ticks == speedup as u32 {
            ticks = 0;

            term.clear_screen().unwrap();

            term.write_line(&format!("Time: {:>9.2}", time_elapsed)).unwrap();

            let mut train_locations = Vec::<(&str, &str)>::new();

            for train in &simulation.signaller.trains {
                term.write_line(&format!("{:>8} | {} |", train.0.name, train.0)).unwrap();
                train_locations.push((&train.0.name, train.2));
            }
    
            for block in simulation.signaller.network.all_edges() {
                let mut colour: &Style = &r;
                let mut train: &str = "";
                
                match &block.2.lock().unwrap().block_type {
                    block::BlockType::Track { signal } => {
                        match signal.colour {
                            SignalColour::Red => colour = &r,
                            SignalColour::DoubleYellow => colour = &dy,
                            SignalColour::Yellow => colour = &y,
                            SignalColour::Green => colour = &g,
                        }
                    },
                    block::BlockType::Station { platforms } => {
                        todo!();
                    },
                }
    
                for loc in &train_locations {
                    if loc.1 == block.1 {
                        train = loc.0;
                    }
                }
    
                term.write_str(&format!("{:>7} {} ", train, colour.apply_to(block.1))).unwrap();
            }
        }
        
        time_elapsed += time_step;
    }


}


