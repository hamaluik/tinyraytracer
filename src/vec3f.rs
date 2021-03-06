#[derive(Default, Clone, Debug, Copy)]
pub struct Vec3f(pub f64, pub f64, pub f64);

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f(x, y, z)
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let scale: f64 = if self.0.max(0.0).max(self.1).max(self.2) > 1.0 {
            1.0 / self.0.max(0.0).max(self.1).max(self.2)
        }
        else {
            1.0
        };

        [
            ((self.0 * scale).max(0.0).min(1.0) * 255f64) as u8,
            ((self.1 * scale).max(0.0).min(1.0) * 255f64) as u8,
            ((self.2 * scale).max(0.0).min(1.0) * 255f64) as u8,
        ]
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn normalized(&self) -> Vec3f {
        let len = self.length();
        Vec3f(
            self.0 / len,
            self.1 / len,
            self.2 / len,
        )
    }

    pub fn add(&self, other: &Vec3f) -> Vec3f {
        Vec3f(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }

    pub fn sub(&self, other: &Vec3f) -> Vec3f {
        Vec3f(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }

    pub fn mult_scalar(&self, scalar: f64) -> Vec3f {
        Vec3f(
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
        )
    }
}

impl From<(f64, f64, f64)> for Vec3f {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Vec3f(tuple.0, tuple.1, tuple.2)
    }
}