use super::vec3f::Vec3f;

pub trait Intersectable {
    fn ray_intersect(&self, origin: &Vec3f, direction: &Vec3f) -> bool;
}