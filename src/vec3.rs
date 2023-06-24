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
        let x = self.e0;
        let y = self.e1;
        let z = self.e2;

        format!("{x:.num_digits$} {y:.num_digits$} {z:.num_digits$}")
    }
}