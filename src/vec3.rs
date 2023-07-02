use rand::Rng;

pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            e0: x,
            e1: y,
            e2: z
        }
    }

    pub fn add(&self, rhs: &Vec3) -> Self {
        Self {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2
        }
    }

    pub fn hadamard(&self, rhs: &Vec3) -> Self {
        Self {
            e0: self.e0 * rhs.e0,
            e1: self.e1 * rhs.e1,
            e2: self.e2 * rhs.e2
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.e0 * rhs.e0 +
            self.e1 * rhs.e1 +
            self.e2 * rhs.e2
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self {
            e0: self.e1 * rhs.e2 -
                self.e2 * rhs.e1,
            e1: self.e2 * rhs.e0 -
                self.e0 * rhs.e2,
            e2: self.e0 * rhs.e1 -
                self.e1 * rhs.e0
        }
    }

    pub fn scalar_add(&self, rhs: f64) -> Self {
        Self {
            e0: self.e0 + rhs,
            e1: self.e1 + rhs,
            e2: self.e2 +rhs
        }
    }

    pub fn scalar_mult(&self, rhs: f64) -> Self {
        Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn to_string(&self, num_digits: usize) -> String {
        let x: i32 = self.e0.floor() as i32;
        let y: i32 = self.e1.floor() as i32;
        let z: i32 = self.e2.floor() as i32;

        format!("{x:.num_digits$} {y:.num_digits$} {z:.num_digits$}")
    }

    pub fn unit_vector(&self) -> Self {
        self.scalar_mult(1.0 / self.length())
    }

    pub fn get_x(&self) -> f64 {
        self.e0
    }
    
    pub fn get_y(&self) -> f64 {
        self.e1
    }

    pub fn get_z(&self) -> f64 {
        self.e2
    }

    pub fn subtract(&self, rhs: &Vec3) -> Self {
        self.add(&rhs.scalar_mult(-1.0))
    }
    
    pub fn duplicate(&self) -> Self {
        Self {
            e0: self.get_x(),
            e1: self.get_y(),
            e2: self.get_z()
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            e0: rng.gen::<f64>(),
            e1: rng.gen::<f64>(),
            e2: rng.gen::<f64>()
        }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            e0: rng.gen_range(min..max),
            e1: rng.gen_range(min..max),
            e2: rng.gen_range(min..max)
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let tst_vec3: Self = Self::random_range(-1.0, 1.0);
            if tst_vec3.length_squared() < 1.0 {
                return tst_vec3;
            }
        }
    }
    
    pub fn random_unit_in_unit_sphere() -> Self {
        Self::unit_vector(&Self::random_in_unit_sphere())
    }

    pub fn random_in_unit_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere: Vec3 = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0 {
            return in_unit_sphere;
        }
        
        return in_unit_sphere.scalar_mult(-1.0);
    }
}