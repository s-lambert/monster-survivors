pub fn lerp(x: f32, y: f32, t: f32) -> f32 {
    (1.0 - t) * x + t * y
}

pub fn inverse_lerp(x: f32, y: f32, v: f32) -> f32 {
    (v - x) / (y - x)
}

pub fn remap(input_min: f32, input_max: f32, output_min: f32, output_max: f32, value: f32) -> f32 {
    let t = inverse_lerp(input_min, input_max, value);
    lerp(output_min, output_max, t)
}
