use crate::include::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius
        }
    }
}

impl Hitable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max:f64,
        rec: &mut HitRecord
    ) -> bool {
        let oc: Vec3 = r.get_origin().subtract(&self.center);
    
        let a: f64 = r.get_direction().length_squared();
        let half_b: f64 = oc.dot(&r.get_direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
    
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (- half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }
        
        rec.set_t(root);
        rec.set_point(r.at(rec.get_t()));
        let outward_normal: Vec3 = rec.get_point()
            .subtract(&self.center)
            .scalar_mult(1.0 / self.radius);
        rec.set_face_normal(r, &outward_normal);
        
        return true;
    }
}