mod sensors;
mod analytics;
mod model;

use model::SensorReading;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let mut ph_buf = Vec::new();
    let mut cond_buf = Vec::new();
    let mut temp_buf = Vec::new();

    loop {
        let raw_ph = sensors::sample_ph();
        let raw_conductivity = sensors::sample_conductivity();
        let raw_temp = sensors::sample_temperature();

        let ph = analytics::moving_average(&mut ph_buf, raw_ph, 10);
        let conductivity = analytics::moving_average(&mut cond_buf, raw_conductivity, 10);
        let temperature = analytics::moving_average(&mut temp_buf, raw_temp, 10);

        let contaminated = analytics::detect_contamination(ph, conductivity);

        let reading = SensorReading {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            ph,
            conductivity,
            temperature,
        };

        let json = serde_json::to_string(&reading).unwrap();
        println!("{} {}", json, if contaminated { "<ALERT>" } else { "" });

        tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    }
}
