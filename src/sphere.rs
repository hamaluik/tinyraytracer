use super::vec3f::Vec3f;
use super::intersectable::{Intersectable, Intersection};
use super::material::Material;

pub struct Sphere<'a> {
    pub centre: Vec3f,
    pub radius: f64,
    pub material: &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn new(centre: Vec3f, radius: f64, material: &'a Material) -> Sphere<'a> {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl<'a> Intersectable for Sphere<'a> {
    fn ray_intersect(&self, origin: &Vec3f, direction: &Vec3f) -> Option<Intersection> {
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
        
        let point = direction.mult_scalar(t0).add(origin);
        let normal = point.sub(&self.centre).normalized();

        Some(Intersection {
            distance: t0,
            point,
            normal,
        })
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
