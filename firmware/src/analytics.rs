pub fn moving_average(buffer: &mut Vec<f32>, new_val: f32, size: usize) -> f32 {
    buffer.push(new_val);
    if buffer.len() > size {
        buffer.remove(0);
    }
    buffer.iter().sum::<f32>() / buffer.len() as f32
}

pub fn detect_contamination(ph: f32, conductivity: f32) -> bool {
    ph < 6.5 || ph > 8.5 || conductivity > 750.0
}
