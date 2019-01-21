use super::vec3f::Vec3f;
use super::intersectable::Intersectable;

pub struct Sphere {
    pub centre: Vec3f,
    pub radius: f64
}

impl Sphere {
    pub fn new(centre: Vec3f, radius: f64) -> Sphere {
        Sphere {
            centre,
            radius,
        }
    }
}

impl Intersectable for Sphere {
    fn ray_intersect(&self, origin: Vec3f, direction: Vec3f) -> bool {
        let line = self.centre.clone() - origin;
        let tca = line.dot(&direction);
        let d2 = line.dot(&line) - tca * tca;

        if d2 > self.radius * self.radius {
            return false;
        }

        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0f64 {
            t0 = t1;
        }
        if t0 < 0f64 {
            return false;
        }

        true
    }
}