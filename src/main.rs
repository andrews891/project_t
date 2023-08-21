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

use crate::simulation::Simulation;

const GRAVITY: f32 = 9.81;

fn main() {
    if cfg!(feature = "logging") {
        env_logger::init();
    }

    let duration = 40000.0;
    let delta_time = 0.01;
    let ticks_per_update = 5;
    let speedup = 50.0;
    
    let simulation = Simulation::new(duration, delta_time, ticks_per_update, speedup);

    simulation.run();
}