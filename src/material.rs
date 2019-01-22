use super::vec3f::Vec3f;

pub struct Material {
    pub diffuse: Vec3f,
    pub albedo: Vec3f,
    pub specular_exponent: f64,
}