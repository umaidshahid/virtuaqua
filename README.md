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

<!-- ## 📦 Project Structure

virtuaqua/
├─ firmware/ # Simulated embedded firmware core
│ ├─ sensors.rs # Virtual sensor drivers
│ ├─ analytics.rs # Signal smoothing & anomaly detection
│ └─ model.rs # Telemetry data structure
├─ dashboard/ # Simple host-side runner
└─ Cargo.toml # Workspace configuration

--- -->

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