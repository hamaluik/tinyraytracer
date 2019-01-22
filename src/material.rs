use super::vec3f::Vec3f;

pub struct Material {
    pub diffuse: Vec3f,
    pub albedo: (f64, f64),
    pub specular_exponent: f64,
}