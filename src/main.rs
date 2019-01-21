use std::fs;
use std::io;
use std::io::prelude::*;

mod vec3f;
mod intersectable;
mod sphere;

fn cast_ray(origin: &vec3f::Vec3f, direction: &vec3f::Vec3f, object: &intersectable::Intersectable) -> vec3f::Vec3f {
    if !object.ray_intersect(origin, direction) {
        return vec3f::Vec3f::new(0.2, 0.7, 0.8);
    }
    return vec3f::Vec3f::new(0.4, 0.4, 0.3);
}

fn main() -> Result<(), io::Error> {
    let width = 1024;
    let height = 768;
    let fov = std::f64::consts::FRAC_PI_2;
    let mut framebuffer: Vec<vec3f::Vec3f> = vec![vec3f::Vec3f::default(); width * height];

    let sphere = sphere::Sphere::new(
        vec3f::Vec3f::new(-3f64, 0f64, -16f64),
        2f64
    );

    for j in 0..height {
        for i in 0..width {
            let x = 2f64 * (i as f64 + 0.5f64) / (width as f64 - 1f64) * (fov / 2f64).tan() * (width as f64) / (height as f64);
            let y = -2f64 * (j as f64 + 0.5f64) / (height as f64 - 1f64) * (fov / 2f64).tan();
            let dir = vec3f::Vec3f::new(x, y, -1f64).normalized();

            framebuffer[i * j + width] = cast_ray(&vec3f::Vec3f::new(0f64, 0f64, 0f64), &dir, &sphere);
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
