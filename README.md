# Virtuaqua

**Virtual water-quality sensing firmware written in Rust.**

Virtuaqua simulates a real-time embedded firmware environment for water quality analysis.  
It streams synthetic pH, conductivity, and temperature values, applies noise filtering,
and detects contamination conditions in real-time — similar to how environmental sensors are used in industrial water monitoring systems.

This project explores:

- Real-time data acquisition loops
- Signal smoothing and data quality filtering
- Threshold-based anomaly detection
- Rust async firmware architecture patterns

> Built as a learning and exploration project while researching real-time water analytics systems.

---

## 💡 Motivation

This project was built to better understand the challenges of real-time water quality monitoring — particularly how noisy analog signals are conditioned, interpreted, and translated into actionable insights.

---

## Features

| Feature | Description |
|--------|-------------|
| **Sensor simulation** | Generates noisy pH, conductivity, and temperature samples |
| **Moving average filtering** | Smooths sensor noise over a rolling window |
| **Contamination detection** | Flags abnormal readings (outside safe water quality ranges) |
| **Real-time streaming output** | Frames readings as JSON packets suitable for logging or ingestion |
| **Multi-binary Rust workspace** | Structured similar to embedded firmware + host tooling layouts |

---

## 🎛️ Sensor Simulation Approach

Since this project runs without physical hardware, sensor readings are generated using controlled randomness. Each measurement starts from a realistic baseline value and adds small, natural variations to mimic:

- **Electronic noise** from ADC sampling
- **Sensor drift** over time
- **Environmental changes** (e.g., temperature effects)

For example:

```rust
// Example of simulated pH reading
7.0 + rand::thread_rng().gen_range(-0.2..0.2)
This produces values that "wiggle" the same way real sensor signals do, which allows us to:

Apply moving average filtering to smooth noisy readings

Test contamination detection logic under realistic conditions

Develop firmware structure before actual hardware is available

---

## 🚀 Running the Simulation

Make sure Rust is installed:  
<https://rustup.rs>

Then simply:

cargo run -p dashboard

You will see real-time telemetry output like:

{"timestamp": 1762355610, "ph": 7.01, "conductivity": 495.4, "temperature": 25.1}
{"timestamp": 1762355611, "ph": 8.61, "conductivity": 802.3, "temperature": 26.0} ⚠️ CONTAMINATION DETECTED


⚠️ CONTAMINATION DETECTED indicates abnormal readings based on defined thresholds.

🔬 Contamination Logic
ph < 6.5  ||  ph > 8.5
conductivity > 750.0


These are simplified WHO/EPA guideline boundaries for potable water.