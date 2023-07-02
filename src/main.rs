mod vec3;
mod color_tools;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;

mod include {
    pub use std::io::Write;
    pub use rand::Rng;
    pub use crate::vec3::*;
    pub use crate::color_tools::*;
    pub use crate::ray::*;
    pub use crate::hitable::*;
    pub use crate::sphere::*;
    pub use crate::hitable_list::*;
    pub use crate::camera::*;
    pub const RESOLUTIONSCALE: f64 = 2.0;
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

    return vec_1.scalar_mult(1.0 - t).add(&vec_2.scalar_mult(t));
}

fn main() {
    let mut rng = rand::thread_rng();

    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_width: i32 = 400 * (RESOLUTIONSCALE as i32);
    let img_height: i32 = ((img_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;

    let mut world: HitableList = HitableList::new();

    world.add(
        Sphere::new(
            Vec3::new(
                0.0 * RESOLUTIONSCALE,
                0.0 * RESOLUTIONSCALE,
                -1.0 * RESOLUTIONSCALE
            ),
            0.5 * RESOLUTIONSCALE
        )
    );

    world.add(
        Sphere::new(
            Vec3::new(
                0.0 * RESOLUTIONSCALE,
                -100.5 * RESOLUTIONSCALE,
                -1.0 * RESOLUTIONSCALE
            ),
            100.0 * RESOLUTIONSCALE
        )
    );

    let viewport_height: f64 = 2.0;
    let focal_length: f64 = 1.0;
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let cam = Camera::new(aspect_ratio, viewport_height, focal_length, origin);


    println!("P3\n{img_width} {img_height}\n255");

    for inv_row in 1..=img_height {
        for col in 1..=img_width {
            let row: i32 = img_height - inv_row;
            
            eprint!("\rRows left: {}        ", row);

            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for _i in 1..=samples_per_pixel {
                let u: f64 = (rng.gen::<f64>() + col as f64) / (img_width as f64 - 1.0);
                let v: f64 = (rng.gen::<f64>() + row as f64) / (img_height as f64 - 1.0);
                let r: Ray = cam.get_ray(u, v);
                pixel_color = pixel_color.add(&ray_color(&r, &world));
            }

            println!("{}", write_color(pixel_color, samples_per_pixel));
            let _ = std::io::stderr().flush();
        }
    }
    eprint!("\n");
}
