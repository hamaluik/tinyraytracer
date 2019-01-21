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
}