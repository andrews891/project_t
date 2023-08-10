use crate::{conversion::*, GRAVITY};

#[macro_export]
macro_rules! class802 {
    ($name:expr) => {
        Train::new(stringify!($name).to_owned(), 300000.0, 700000, 3, 2.5, 3.5, 1.0)
    };
}

#[derive(Debug)]
pub struct Train {
    pub name: String,
    mass: f32,
    power: u32,
    axle_resistance: f32,
    rolling_resistance: f32,
    air_resistance_coefficient: f32,
    max_tractive_effort: u32,
    pub position: f32,
    pub velocity: f32,
    throttle: i16,
    acceleration: f32,
    pub target_velocity: f32,
    pub target_distance: f32,
    max_throttle: i16,
    max_brake: i16,
    emergency_brake: i16,
    emergency: bool,
}

impl Train {
    pub fn new(name: String, mass: f32, power: u32, engines: u32, width: f32, height: f32, acceleration: f32) -> Self {
        return Train {
            name: name,
            mass: mass,
            power: power * engines,
            axle_resistance: 0.002 * mass * GRAVITY, // estimate of axle resistance (less than steel-steel)
            rolling_resistance: 0.0015 * mass * GRAVITY, // steel-steel rolling resistance is 0.1-0.2%
            air_resistance_coefficient: width * height, // requires (* v * v)
            max_tractive_effort: (0.5 * mass * acceleration) as u32, // friction of steel-steel (0.15 for wet, 0.5 for dry)
            position: 0.0,
            velocity: 0.0,
            throttle: 0,
            acceleration: 0.0,
            target_velocity: 0.0,
            target_distance: 0.0,
            max_throttle: 100,
            max_brake: 150,
            emergency_brake: 200,
            emergency: false,
        }
    }

    pub fn update(&mut self, delta_time: f32, control: bool) {
        if !self.emergency && control {
            self.control();
        }
        else {
            if self.velocity.abs() <= 0.1 {
                self.emergency = false;
                self.throttle = 0;
            }
        }

        let mut resistive_force = 0.0;
        
        resistive_force += self.axle_resistance;
        resistive_force += self.rolling_resistance;
        resistive_force += self.air_resistance_coefficient * self.velocity.powf(2.0);

        let propulsion_force = (self.throttle / 100) as f32 * core::cmp::min(self.max_tractive_effort, (self.power as f32 / self.velocity.abs()) as u32) as f32;

        let force = propulsion_force - (self.velocity.signum() * resistive_force);

        self.acceleration = force / self.mass;

        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
    }

    fn control(&mut self) {
        if self.target_distance < 0.0 { // immediate + emergency brake
            dbg!("EMERGENCY");
            self.throttle = -self.emergency_brake;
            self.emergency = true;
        }
        else { // evaluate required amount of acceleration
            //let 
            let target_acceleration = (self.target_velocity.powf(2.0) - self.velocity.powf(2.0)) / (2.0 * (self.target_distance - 5.0));
            // v^2 = u^2 + 2as
            // a = (v^2 - u^2 / 2s)
            if self.acceleration > target_acceleration {
                if false{//self.throttle == -self.max_brake {
                    dbg!(&self, target_acceleration);
                    self.throttle = -self.emergency_brake;
                    self.emergency = true;
                }
                else {
                    self.throttle = std::cmp::max(self.throttle - 10, -self.max_brake);
                }
            }
            else if self.acceleration < target_acceleration {
                self.throttle = std::cmp::min(self.throttle + 10, self.max_throttle);
            }
        }
    }
}

impl std::fmt::Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vel: {:>6.2}m/s : {:>6.2}mph | Pos {:>7.2}m | Acc {:>6.3} | Thr {:>4} | Target {:>8.2}m/s in {:>8.2}m", self.velocity, convert_to_mph(self.velocity), self.position, self.acceleration, self.throttle, self.target_velocity, self.target_distance)
    }
}