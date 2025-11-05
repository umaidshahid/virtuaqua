mod logger;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() {
    println!("\nStarting firmware stream and logging to data.csv ...\n");

    let mut child = Command::new("cargo")
        .args(["run", "-p", "firmware"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run firmware");

    let stdout = child.stdout.take().expect("No stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.unwrap();

        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&line) {
            let timestamp = value["timestamp"].as_u64().unwrap();
            let ph = value["ph"].as_f64().unwrap() as f32;
            let conductivity = value["conductivity"].as_f64().unwrap() as f32;
            let temperature = value["temperature"].as_f64().unwrap() as f32;

            logger::write_csv(timestamp, ph, conductivity, temperature);
        }

        println!("{line}");
    }
}
