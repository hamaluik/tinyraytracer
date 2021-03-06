use super::vec3f::Vec3f;
use super::material::Material;

#[derive(Clone, Debug, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub point: Vec3f,
    pub normal: Vec3f,
}

pub trait Intersectable {
    fn ray_intersect(&self, origin: &Vec3f, direction: &Vec3f) -> Option<Intersection>;
    fn material(&self) -> &Material;
}

