use crate::include::*;

pub fn clamp(x: f64, min_bound: f64, max_bound: f64) -> f64 {
    let mut clamped: f64 = x;

    if x < min_bound {
        clamped = min_bound;
    } else if x > max_bound {
        clamped = max_bound;
    }

    return clamped;
}

pub fn write_color(color: Vec3, samples_per_pixel: i32) -> String {
    let scale: f64 = 1.0 / (samples_per_pixel as f64);

    let r: f64 = 255.999 * clamp(color.get_x() * scale, 0.0, 1.0);
    let g: f64 = 255.999 * clamp(color.get_y() * scale, 0.0, 1.0);
    let b: f64 = 255.999 * clamp(color.get_z() * scale, 0.0, 1.0);

    let color_to_write: Vec3 = Vec3::new(r, g, b);

    color_to_write.to_string(0)
}