use crate::include::*;

pub struct HitableList {
    objects:    Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::<Box<dyn Hitable>>::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    
    pub fn add(&mut self, hitable: impl Hitable + 'static) {
        self.objects.push(Box::new(hitable))
    }
}

impl Hitable for HitableList {
    fn hit(
        &self,
        r:      &Ray,
        t_min:  f64,
        t_max:  f64,
        rec:    &mut HitRecord
    ) -> bool {
        let mut temp_rec: HitRecord = HitRecord{
            point:      Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 },
            normal:     Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 },
            t:          0.0,
            front_face: false
        };
        let mut closest_so_far: f64 = t_max;
        let mut hit_anything: bool = false;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                if temp_rec.get_t() < closest_so_far {
                    closest_so_far = temp_rec.get_t();
                }
            }
        }
        
        rec.set_all(&temp_rec);
        
        return hit_anything;
    }
}