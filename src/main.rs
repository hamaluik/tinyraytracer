use std::fs;
use std::io;
use std::io::prelude::*;

mod vec3f;
mod intersectable;
mod sphere;

fn main() -> Result<(), io::Error> {
    let width = 1024;
    let height = 768;
    let mut framebuffer: Vec<vec3f::Vec3f> = vec![vec3f::Vec3f::default(); width * height];

    for j in 0..height {
        for i in 0..width {
            framebuffer[i + j * width] = vec3f::Vec3f::new(
                (j as f64) / (height as f64),
                (i as f64) / (height as f64),
                0f64,
            );
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
