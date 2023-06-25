use crate::include::*;

pub struct HitRecord {
    pub point:      Vec3,
    pub normal:     Vec3,
    pub t:          f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn get_point(&self) -> Vec3 {
        return self.point;
    }

    pub fn get_normal(&self) -> Vec3 {
        return self.normal;
    }

    pub fn get_t(&self) -> f64 {
        return self.t;
    }

    pub fn get_front_face(&self) -> bool {
        return self.front_face;
    }

    pub fn set_point(&mut self, point: Vec3) {
        self.point = point;
    }
    
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }
    
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub fn set_front_face(&mut self, front_face: bool) {
        self.front_face = front_face;
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.set_front_face(r.get_direction().dot(outward_normal) < 0.0);
        if self.get_front_face() {
            self.set_normal(Vec3 {
                e0: outward_normal.get_x(),
                e1: outward_normal.get_y(),
                e2: outward_normal.get_z()
            });
        } else {
            self.set_normal(Vec3 {
                e0: -outward_normal.get_x(),
                e1: -outward_normal.get_y(),
                e2: -outward_normal.get_z()
            });
        }
    }
}

pub trait Hitable {
    fn hit(
        &self,
        r:      &Ray,
        t_min:  f64,
        t_max:  f64,
        rec:    &mut HitRecord
    ) -> bool;
}