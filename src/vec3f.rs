use std::ops;

#[derive(Default, Clone)]
pub struct Vec3f(pub f64, pub f64, pub f64);

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f(x, y, z)
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.0.max(0.0).min(1.0) * 255f64) as u8,
            (self.1.max(0.0).min(1.0) * 255f64) as u8,
            (self.2.max(0.0).min(1.0) * 255f64) as u8,
        ]
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other . 2
    }
}

impl ops::Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }
}