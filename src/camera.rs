use crate::include::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
        origin: Vec3
    ) -> Self {
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let horizontal: Vec3 = Vec3::new(viewport_width, origin.get_y(), origin.get_z());
        let vertical: Vec3 = Vec3::new(origin.get_x(), viewport_height, origin.get_z());
        let lower_left_corner = origin.subtract(
            &(horizontal.scalar_mult(0.5))
        ).subtract(
            &(vertical.scalar_mult(0.5))
        ).subtract(
            &Vec3 { e0: 0.0, e1: 0.0, e2: focal_length }
        );

        Self { origin: origin, lower_left_corner: lower_left_corner, horizontal: horizontal, vertical: vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &(self.origin),
            &(self.lower_left_corner
                .add(&(self.horizontal.scalar_mult(u)))
                .add(&(self.vertical.scalar_mult(v)))
                .subtract(&self.origin))
        )
    }
}
