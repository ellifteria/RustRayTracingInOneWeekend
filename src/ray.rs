use crate::include::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin:     Vec3,
    pub direction:  Vec3
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Self {
        Self {
            origin: Vec3::new(origin.get_x(), origin.get_y(), origin.get_z()),
            direction: Vec3::new(direction.get_x(), direction.get_y(), direction.get_z()),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.add(&self.direction.scalar_mult(t))
    }
    pub fn get_origin(&self) -> Vec3 {
        self.origin.clone()
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction.clone()
    }
}