pub fn f32_to_fixed(v: f32, min: f32, precision: f32) -> u32 {
    ((v - min) / precision) as u32
}

pub fn fixed_to_f32(v: u32, min: f32, precision: f32) -> f32 {
    v as f32 * precision + min
}

pub fn f64_to_fixed(v: f64, min: f64, precision: f64) -> u64 {
    ((v - min) / precision) as u64
}

pub fn fixed_to_f64(v: u64, min: f64, precision: f64) -> f64 {
    v as f64 * precision + min
}
