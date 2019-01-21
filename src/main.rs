use std::fs;
use std::io;
use std::io::prelude::*;

mod vec3f;
mod intersectable;
mod material;
mod sphere;

fn ray_direction(x: usize, y: usize, width: usize, height: usize, field_of_view: f64) -> vec3f::Vec3f {
    let px: f64 = (2.0 * ((x as f64 + 0.5) / (width as f64)) - 1.0) * (field_of_view / 2.0).tan() * (width as f64 / height as f64);
    let py: f64 = (1.0 - 2.0 * ((y as f64 + 0.5) / (height as f64))) * (field_of_view / 2.0).tan();
    vec3f::Vec3f::new(px, py, -1.0).normalized()
}

fn cast_ray<'a>(origin: &vec3f::Vec3f, direction: &vec3f::Vec3f, objects: &'a Vec<&'a intersectable::Intersectable>) -> Option<&'a vec3f::Vec3f> {
    let mut closest_colour = None;
    let mut closest_intersect: f64 = std::f64::MAX;

    for object in objects.iter() {
        if let Some(intersection) = object.ray_intersect(origin, direction) {
            if intersection.distance < closest_intersect {
                closest_colour = Some(&object.material().diffuse);
                closest_intersect = intersection.distance;
            }
        }
    }
    
    closest_colour
}

fn main() -> Result<(), io::Error> {
    let width = 512;
    let height = 512;
    let fov = std::f64::consts::FRAC_PI_2;
    let mut framebuffer: Vec<vec3f::Vec3f> = vec![vec3f::Vec3f::default(); width * height];

    let background = vec3f::Vec3f::new(0.2, 0.7, 0.8);
    let ivory = material::Material { diffuse: vec3f::Vec3f::new(0.4, 0.4, 0.3) };
    let red_rubber = material::Material { diffuse: vec3f::Vec3f::new(0.3, 0.1, 0.1) };

    let objects = vec![
        sphere::Sphere::new(vec3f::Vec3f::new(-3.0, 0.0, -16.0), 2.0, &ivory),
        sphere::Sphere::new(vec3f::Vec3f::new(-1.0, -1.5, -12.0), 2.0, &red_rubber),
        sphere::Sphere::new(vec3f::Vec3f::new(1.5, -0.5, -18.0), 3.0, &red_rubber),
        sphere::Sphere::new(vec3f::Vec3f::new(7.0, 5.0, -18.0), 4.0, &ivory),
    ];
    let objects = objects.iter().map(|o| o as &intersectable::Intersectable).collect();

    let origin = vec3f::Vec3f::new(0f64, 0f64, 0f64);
    for j in 0..height {
        for i in 0..width {
            let dir = ray_direction(i, j, width, height, fov);
            framebuffer[i + (j * width)] = match cast_ray(&origin, &dir, &objects) {
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