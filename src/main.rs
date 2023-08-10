#[macro_use] mod train;
mod surface;
mod conversion;
mod simulation;
mod signaller;
mod signal;
mod io;

use crate::train::*;
use crate::surface::*;
use crate::simulation::*;
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

    let duration = 4000.0;
    let time_step = 0.01;
    let speedup = 25.0;

    let mut time_elapsed = 0.0;

    let mut interval = time::interval(time::Duration::from_secs_f32(time_step / speedup));
    
    simulation.signaller.propagate_signal("I", "SIGNALLER", SignalColour::Red);

    dbg!(&simulation);
    
    while time_elapsed < duration {
        interval.tick().await;
        simulation.time_step(time_step);

        term.clear_screen().unwrap();

        term.write_line(&format!("Time: {:>9.2}", time_elapsed)).unwrap();

        let mut train_locations = Vec::<(&str, &str)>::new();

        for train in &simulation.signaller.trains {
            term.write_line(&format!("{:>8} | {} ({}) |", train.0.name, train.0, train.2)).unwrap();
            train_locations.push((&train.0.name, train.2));
        }

        for signal in simulation.signaller.network.all_edges() {
            let colour: &Style;
            let mut train: &str = "";
            
            match signal.2.lock().unwrap().colour {
                SignalColour::Red => colour = &r,
                SignalColour::DoubleYellow => colour = &dy,
                SignalColour::Yellow => colour = &y,
                SignalColour::Green => colour = &g,
            }

            for loc in &train_locations {
                if loc.1 == signal.1 {
                    train = loc.0;
                }
            }

            term.write_str(&format!("{:>7} {} ", train, colour.apply_to(signal.1))).unwrap();
        }

        // term.write_line(&format!("{}",
        // simulation.signaller.network.edge_weight("A", "B").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("B", "C").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("C", "D").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("D", "E").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("E", "F").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("F", "G").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("G", "H").unwrap().lock().unwrap().colour,
        // simulation.signaller.network.edge_weight("H", "I").unwrap().lock().unwrap().colour)).unwrap();
        
        time_elapsed += time_step;
    }


}


