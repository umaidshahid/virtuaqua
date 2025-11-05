use std::process::Command;

fn main() {
    println!("Launching firmware stream...\n");
    Command::new("cargo")
        .args(["run", "-p", "firmware"])
        .status()
        .expect("Failed to run firmware");
}
