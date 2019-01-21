use super::vec3f::Vec3f;
use super::sphere::Sphere;
use super::intersectable::Intersectable;

#[test]
fn ray_should_intersect_sphere_direct() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(0.0, 0.0, -1.0);
    let sphere: Sphere = Sphere::new(
        Vec3f::new(0.0, 0.0, -10.0),
        1.0
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert_eq!(intersects.is_some(), true);
}

#[test]
fn ray_should_not_intersect_sphere_behind() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(0.0, 0.0, -1.0);
    let sphere: Sphere = Sphere::new(
        Vec3f::new(0.0, 0.0, 10.0),
        1.0
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert_eq!(intersects, None);
}

#[test]
fn ray_should_intersect_sphere_centre_deflected() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(1.0, 0.0, -1.0).normalized();
    let sphere: Sphere = Sphere::new(
        Vec3f::new(5.0, 0.0, -5.0),
        1.0
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert_eq!(intersects.is_some(), true);
}

#[test]
fn ray_should_intersect_sphere_centre_deflected_2() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(1.0, 0.0, -1.0).normalized();
    let sphere: Sphere = Sphere::new(
        Vec3f::new(4.0, 0.0, -5.0),
        1.0
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert_eq!(intersects.is_some(), true);
}

#[test]
fn ray_should_miss() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = super::ray_direction(0, 0, 512, 512, std::f64::consts::FRAC_PI_2);
    let sphere: Sphere = Sphere::new(
        Vec3f::new(0.0, 0.0, -10.0),
        1.0
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert_eq!(intersects, None);
}