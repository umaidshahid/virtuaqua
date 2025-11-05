use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SensorReading {
    pub timestamp: u64,
    pub ph: f32,
    pub conductivity: f32,
    pub temperature: f32,
}
