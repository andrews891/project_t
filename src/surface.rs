use crate::GRAVITY;

pub struct Surface {
    rolling_coefficient: f32,
    friction_coefficient: f32,
}

impl Surface {
    pub fn new(rolling_coefficient: f32) -> Self {
        return Surface {
            rolling_coefficient: rolling_coefficient,
            friction_coefficient: 0.0,
        }
    }

    pub fn calculate_friction(&self, mass: f32) -> f32 {
        return mass * GRAVITY * self.rolling_coefficient;
    }
}