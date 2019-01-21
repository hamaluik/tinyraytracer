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

fn cast_ray(origin: &vec3f::Vec3f, direction: &vec3f::Vec3f, object: &intersectable::Intersectable) -> vec3f::Vec3f {
    let res = object.ray_intersect(origin, direction);
    if res.is_some() {
        vec3f::Vec3f::new(1.0, 1.0, 1.0)
    }
    else {
        vec3f::Vec3f::new(0.2, 0.7, 0.8)
    }
}

fn main() -> Result<(), io::Error> {
    let width = 1024;
    let height = 768;
    let fov = std::f64::consts::FRAC_PI_4;
    let mut framebuffer: Vec<vec3f::Vec3f> = vec![vec3f::Vec3f::default(); width * height];

    let sphere = sphere::Sphere::new(
        vec3f::Vec3f::new(0f64, 0f64, -16f64),
        2f64
    );

    let origin = vec3f::Vec3f::new(0f64, 0f64, 0f64);
    for j in 0..height {
        for i in 0..width {
            let dir = ray_direction(i, j, width, height, fov);
            framebuffer[i + (j * width)] = cast_ray(&origin, &dir, &sphere);
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