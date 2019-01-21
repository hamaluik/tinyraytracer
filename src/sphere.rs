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
    fn ray_intersect(&self, origin: &Vec3f, direction: &Vec3f) -> Option<f64> {
        let l = self.centre.sub(origin);
        let tca = l.dot(direction);
        let d2 = l.length_squared() - (tca*tca);
        if d2 > self.radius * self.radius {
            return None;
        }

        let thc = ((self.radius * self.radius) - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return None;
        }
        Some(t0)
    }
}