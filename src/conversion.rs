pub fn convert_to_mph(mps: f32) -> f32 {
    return mps * 2.237;
}

pub fn convert_to_mps(mph: f32) -> u32 {
    return (mph / 2.237).round() as u32;
}