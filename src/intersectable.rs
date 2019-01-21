use super::vec3f::Vec3f;

pub struct Intersection {
    pub distance: f64,
    pub point: Vec3f,
    pub normal: Vec3f,
}

pub trait Intersectable {
    fn ray_intersect(&self, origin: &Vec3f, direction: &Vec3f) -> Option<Intersection>;
    fn diffuse_colour(&self) -> &Vec3f;
}