use std::fs;
use std::io;
use std::io::prelude::*;

mod vec3f;
mod intersectable;
mod material;
mod sphere;
mod light;

fn ray_direction(x: usize, y: usize, width: usize, height: usize, field_of_view: f64) -> vec3f::Vec3f {
    let px: f64 = (2.0 * ((x as f64 + 0.5) / (width as f64)) - 1.0) * (field_of_view / 2.0).tan() * (width as f64 / height as f64);
    let py: f64 = (1.0 - 2.0 * ((y as f64 + 0.5) / (height as f64))) * (field_of_view / 2.0).tan();
    vec3f::Vec3f::new(px, py, -1.0).normalized()
}

fn reflect(i: &vec3f::Vec3f, n: &vec3f::Vec3f) -> vec3f::Vec3f {
    i.sub(&n.mult_scalar(2.0).mult_scalar(i.dot(&n)))
}

fn scene_intersect<'a>(origin: &vec3f::Vec3f, direction: &vec3f::Vec3f, objects: &Vec<&'a intersectable::Intersectable>) -> Option<(intersectable::Intersection, &'a material::Material)> {
    let mut material: Option<&material::Material> = None;
    let mut closest_intersection: Option<intersectable::Intersection> = None;

    for object in objects.iter() {
        if let Some(intersection) = object.ray_intersect(origin, direction) {
            if closest_intersection.is_none() || (closest_intersection.is_some() && intersection.distance < closest_intersection.unwrap().distance) {
                material = Some(object.material());
                closest_intersection = Some(intersection);
            }
        }
    }

    if closest_intersection.is_none() {
        return None;
    }
    Some((closest_intersection.unwrap(), material.unwrap()))
}

fn cast_ray<'a>(origin: &vec3f::Vec3f, direction: &vec3f::Vec3f, objects: &'a Vec<&'a intersectable::Intersectable>, lights: &Vec<light::Light>, depth: usize) -> Option<vec3f::Vec3f> {
    if depth > 4 {
        return None;
    }

    let (intersection, material) = match scene_intersect(origin, direction, objects) {
        Some((i, m)) => (i, m),
        None => return None,
    };

    let reflect_direction = reflect(&direction, &intersection.normal).normalized();
    let reflect_origin: vec3f::Vec3f = if reflect_direction.dot(&intersection.normal) < 0.0 {
        intersection.point.sub(&intersection.normal.mult_scalar(1e-3))
    }
    else {
        intersection.point.add(&intersection.normal.mult_scalar(1e-3))
    };
    let reflect_colour = cast_ray(&reflect_origin, &reflect_direction, objects, lights, depth + 1).unwrap_or(vec3f::Vec3f::new(0.0, 0.0, 0.0));

    let mut diffuse_light_intensity: f64 = 0.0;
    let mut specular_light_intensity: f64 = 0.0;
    for light in lights.iter() {
        let light_direction = light.position.sub(&intersection.point).normalized();
        let light_distance = light.position.sub(&intersection.point).length();

        let shadow_origin: vec3f::Vec3f = if light_direction.dot(&intersection.normal) < 0.0 {
            intersection.point.sub(&intersection.normal.mult_scalar(1e-3))
        }
        else {
            intersection.point.add(&intersection.normal.mult_scalar(1e-3))
        };
        if let Some((shadow_intersect, _)) = scene_intersect(&shadow_origin, &light_direction, objects) {
            if shadow_intersect.point.sub(&shadow_origin).length() < light_distance {
                continue;
            }
        }

        diffuse_light_intensity += light.intensity * (light_direction.dot(&intersection.normal)).max(0.0);
        specular_light_intensity += (reflect(&light_direction.mult_scalar(-1.0), &intersection.normal).mult_scalar(-1.0).dot(direction)).max(0.0).powf(material.specular_exponent) * light.intensity;
    }
    
    Some(
        material.diffuse.mult_scalar(diffuse_light_intensity * material.albedo.0)
        .add(&vec3f::Vec3f::new(1.0, 1.0, 1.0).mult_scalar(specular_light_intensity * material.albedo.1))
        .add(&reflect_colour.mult_scalar(material.albedo.2))
    )
}

fn main() -> Result<(), io::Error> {
    let width = 1920;
    let height = 1080;
    let fov = std::f64::consts::FRAC_PI_2;
    let mut framebuffer: Vec<vec3f::Vec3f> = vec![vec3f::Vec3f::default(); width * height];

    let background = vec3f::Vec3f::new(0.2, 0.7, 0.8);
    let ivory = material::Material { diffuse: vec3f::Vec3f::new(0.4, 0.4, 0.3), albedo: vec3f::Vec3f::new(0.6, 0.3, 0.1), specular_exponent: 50.0 };
    let red_rubber = material::Material { diffuse: vec3f::Vec3f::new(0.3, 0.1, 0.1), albedo: vec3f::Vec3f::new(0.9, 0.1, 0.0), specular_exponent: 10.0 };
    let mirror = material::Material { diffuse: vec3f::Vec3f::new(0.0, 0.0, 0.0), albedo: vec3f::Vec3f::new(1.0, 1.0, 1.0), specular_exponent: 100.0 };

    let objects = vec![
        sphere::Sphere::new(vec3f::Vec3f::new(-3.0, 0.0, -16.0), 2.0, &ivory),
        sphere::Sphere::new(vec3f::Vec3f::new(-1.0, -1.5, -12.0), 2.0, &mirror),
        sphere::Sphere::new(vec3f::Vec3f::new(1.5, -0.5, -18.0), 3.0, &red_rubber),
        sphere::Sphere::new(vec3f::Vec3f::new(7.0, 5.0, -18.0), 4.0, &mirror),
    ];
    let objects = objects.iter().map(|o| o as &intersectable::Intersectable).collect();

    let lights = vec![
        light::Light {
            position: vec3f::Vec3f::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        light::Light {
            position: vec3f::Vec3f::new(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        light::Light {
            position: vec3f::Vec3f::new(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];

    let origin = vec3f::Vec3f::new(0f64, 0f64, 0f64);
    for j in 0..height {
        for i in 0..width {
            let dir = ray_direction(i, j, width, height, fov);
            framebuffer[i + (j * width)] = match cast_ray(&origin, &dir, &objects, &lights, 0) {
                Some(c) => c.clone(),
                None => background.clone(),
            };
        }
    }

    let file = fs::File::create("out.ppm")?;
    let mut buf = io::BufWriter::new(file);
    buf.write_fmt(format_args!("P6\n{} {}\n255\n", width, height))?;
    for i in 0..(width * height) {
        buf.write(&framebuffer[i].to_rgb())?;
    }

    println!("render saved to out.ppm");
    Ok(())
}

#[cfg(test)] mod tests;