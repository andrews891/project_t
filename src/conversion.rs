pub fn convert_to_mph(mps: f32) -> f32 {
    mps * 2.237
}

pub fn convert_to_mps(mph: u32) -> u32 {
    (mph as f32 / 2.237).round() as u32
}