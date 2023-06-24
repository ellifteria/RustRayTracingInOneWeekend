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

pub fn write_color(color: Vec3) -> String {
    let r: f64 = 255.999 * clamp(color.e0, 0.0, 1.0);
    let g: f64 = 255.999 * clamp(color.e1, 0.0, 1.0);
    let b: f64 = 255.999 * clamp(color.e2, 0.0, 1.0);

    let color_to_write = Vec3::new(r, g, b);

    color_to_write.to_string(0)
}