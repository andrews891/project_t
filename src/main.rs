mod utils {
    pub mod surface;
    pub mod conversion;
    pub mod io;
    pub mod visualiser;
    pub mod bihashmap;
}
#[macro_use] mod infrastructure {
    pub mod signal;
    pub mod block;
    pub mod platform;
    #[macro_use] pub mod train;
}
mod control {
    pub mod signaller;
    pub mod driver;
    pub mod message;
}
mod simulation;





use crate::{simulation::Simulation};

use crate::utils::visualiser::Visualiser;
use tokio::{time, task};

const GRAVITY: f32 = 9.81;

#[tokio::main]
async fn main() {
    let mut simulation = task::spawn_blocking(Simulation::new).await.unwrap();

    let visualiser = Visualiser::new();

    dbg!();

    let duration = 4000.0;
    let time_step = 0.01;
    let speedup = 20.0;

    let mut time_elapsed = 0.0;

    let mut ticks = (speedup - 1.0) as u32;

    let mut interval = time::interval(time::Duration::from_secs_f32(time_step / speedup));

    println!("Initialised");
    
    while time_elapsed < duration {
        ticks += 1;

        interval.tick().await;

        simulation.time_step(time_step).await;

        if ticks == speedup as u32 {
            visualiser.update(time_elapsed, &simulation.drivers, &simulation.signaller.network);
            ticks = 0;
        }
        
        time_elapsed += time_step;
    }

    println!("Done");
}


