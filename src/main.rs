use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), io::Error> {
    let width = 1024;
    let height = 768;
    let mut framebuffer = vec![(0f64, 0f64, 0f64); width * height];

    for j in 0..height {
        for i in 0..width {
            framebuffer[i + j * width] = (
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
        let r = (framebuffer[i].0.max(0.0).min(1.0) * 255f64) as u8;
        let g = (framebuffer[i].1.max(0.0).min(1.0) * 255f64) as u8;
        let b = (framebuffer[i].2.max(0.0).min(1.0) * 255f64) as u8;
        buf.write(&[r, g, b])?;
    }

    println!("render saved to out.ppm");
    Ok(())
}
