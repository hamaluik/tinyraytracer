use std::fs;
use std::io;
use std::io::prelude::*;

mod vec3f;
mod intersectable;
mod sphere;

fn ray_direction(x: usize, y: usize, width: usize, height: usize, field_of_view: f64) -> vec3f::Vec3f {
    let px: f64 = (2.0 * ((x as f64 + 0.5) / (width as f64)) - 1.0) * (field_of_view / 2.0).tan() * (width as f64 / height as f64);
    let py: f64 = (1.0 - 2.0 * ((y as f64 + 0.5) / (height as f64))) * (field_of_view / 2.0).tan();
    vec3f::Vec3f::new(px, py, -1.0).normalized()
}

fn cast_ray<'a>(origin: &vec3f::Vec3f, direction: &vec3f::Vec3f, objects: &Vec<&'a intersectable::Intersectable>) -> Option<&'a vec3f::Vec3f> {
    let mut closest_colour = None;
    let mut closest_intersect: f64 = std::f64::MAX;

    for object in objects.iter() {
        if let Some(dist) = object.ray_intersect(origin, direction) {
            if dist < closest_intersect {
                closest_colour = Some(object.colour());
                closest_intersect = dist;
            }
        }
    }
    
    closest_colour
}

fn main() -> Result<(), io::Error> {
    let width = 1024;
    let height = 768;
    let fov = std::f64::consts::FRAC_PI_4;
    let mut framebuffer: Vec<vec3f::Vec3f> = vec![vec3f::Vec3f::default(); width * height];

    let background = vec3f::Vec3f::new(0.2, 0.7, 0.8);
    let sphere1 = sphere::Sphere::new(
        vec3f::Vec3f::new(0f64, 0f64, -16f64),
        2f64,
        vec3f::Vec3f::new(0.8, 0.6, 0.3)
    );
    let sphere2 = sphere::Sphere::new(
        vec3f::Vec3f::new(2f64, 1f64, -10f64),
        1f64,
        vec3f::Vec3f::new(0.1, 0.6, 0.3)
    );

    let objects = vec![
        &sphere1 as &intersectable::Intersectable,
        &sphere2 as &intersectable::Intersectable,
    ];

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