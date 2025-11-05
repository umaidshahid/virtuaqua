use rand::Rng;

pub fn sample_ph() -> f32 {
    7.0 + rand::thread_rng().gen_range(-0.2..0.2)
}

pub fn sample_conductivity() -> f32 {
    500.0 + rand::thread_rng().gen_range(-30.0..30.0)
}

pub fn sample_temperature() -> f32 {
    25.0 + rand::thread_rng().gen_range(-1.0..1.0)
}
