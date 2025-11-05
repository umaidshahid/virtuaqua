use std::fs::OpenOptions;
use std::io::Write;

pub fn write_csv(timestamp: u64, ph: f32, conductivity: f32, temperature: f32) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.csv")
        .expect("Failed to open data.csv");

    writeln!(file, "{timestamp},{ph},{conductivity},{temperature}")
        .expect("Failed to write to data.csv");
}
