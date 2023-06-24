mod vec3;
mod color_tools;
mod ray;

mod include {
    pub use std::io::Write;
    pub use crate::vec3::*;
    pub use crate::color_tools::*;
    pub use crate::ray::*;
}

use include::*;

fn ray_color(r: &Ray) -> Vec3 {
    let unit_dir = r.direction.unit_vector();
    let t = 0.5 * (unit_dir.get_y() + 1.0);
    
    let vec_1 = Vec3::new(1.0, 1.0, 1.0);
    let vec_2 = Vec3::new(0.5, 0.7, 1.0);

    vec_1.scalar_mult(1.0 - t).add(&vec_2.scalar_mult(t))
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_width = 400;
    let img_height = ((img_width as f64) / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewpost_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewpost_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        .subtract(&horizontal.scalar_mult(0.5))
        .subtract(&vertical.scalar_mult(0.5))
        .add(&Vec3::new(0.0, 0.0, focal_length));

    println!("P3\n{img_width} {img_height}\n255");

    for inv_row in 1..=img_height {
        for col in 1..=img_width {
            eprint!("\rLine: {}    ", col);
            let row = img_height - inv_row;

            let u = (col as f64) / (img_width as f64 - 1.0);
            let v = (row as f64) / (img_height as f64 - 1.0);

            let r = Ray::new(
                &origin,
                &(
                    &lower_left_corner
                        .add(&horizontal.scalar_mult(u))
                        .add(&vertical.scalar_mult(v))
                        .subtract(&origin)));

            let pixel_color = ray_color(&r);

            println!("{}", write_color(pixel_color));
            let _ = std::io::stderr().flush();
        }
    }
    eprint!("\n");
}