mod vec3;
mod color_tools;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;

mod include {
    pub use std::io::Write;
    pub use crate::vec3::*;
    pub use crate::color_tools::*;
    pub use crate::ray::*;
    pub use crate::hitable::*;
    pub use crate::sphere::*;
    pub use crate::hitable_list::*;
}

use include::*;

fn ray_color(r: &Ray, world: &dyn Hitable) -> Vec3 {
    let mut rec: HitRecord = HitRecord{
        point:      Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 },
        normal:     Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 },
        t:          0.0,
        front_face: false
    };

    if world.hit(
        r,
        0.0,
        f64::INFINITY,
        &mut rec
    ) {
        return rec.get_normal()
            .scalar_add(1.0)
            .scalar_mult(0.5);
    }

    let unit_dir: Vec3 = r.get_direction()
        .unit_vector();
    let t: f64 = 0.5 * (unit_dir.get_y() + 1.0);
    
    let vec_1: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let vec_2: Vec3 = Vec3::new(0.5, 0.7, 1.0);

    vec_1.scalar_mult(1.0 - t).add(&vec_2.scalar_mult(t))
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_width: i32 = 400;
    let img_height: i32 = ((img_width as f64) / aspect_ratio) as i32;

    let mut world: HitableList = HitableList::new();

    world.add(
        Sphere::new(
            Vec3::new(
                0.0,
                0.0,
                -1.0
            ),
            0.5
        )
    );
    world.add(
        Sphere::new(
            Vec3::new(
                0.0,
                -100.5,
                -1.0
            ),
            100.0
        )
    );

    let viewport_height: f64 = 2.0;
    let viewpost_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewpost_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = origin
        .subtract(&horizontal.scalar_mult(0.5))
        .subtract(&vertical.scalar_mult(0.5))
        .subtract(&Vec3::new(0.0, 0.0, focal_length));

    println!("P3\n{img_width} {img_height}\n255");

    for inv_row in 1..=img_height {
        for col in 1..=img_width {
            eprint!("\rLine: {}    ", col);
            let row: i32 = img_height - inv_row;

            let u: f64 = (col as f64) / (img_width as f64 - 1.0);
            let v: f64 = (row as f64) / (img_height as f64 - 1.0);

            let r: Ray = Ray::new(
                &origin,
                &(
                    &lower_left_corner
                        .add(&horizontal.scalar_mult(u))
                        .add(&vertical.scalar_mult(v))
                        .subtract(&origin)));

            let pixel_color: Vec3 = ray_color(&r, &world);

            println!("{}", write_color(pixel_color));
            let _ = std::io::stderr().flush();
        }
    }
    eprint!("\n");
}