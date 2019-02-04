#![feature(test)]
extern crate test;

use std::fs;
use std::io;
use std::io::prelude::*;
use rayon::prelude::*;

mod vec3f;
mod intersectable;
mod material;
mod sphere;
mod light;
mod grid;

pub use crate::vec3f::Vec3f;
pub use crate::intersectable::{ Intersectable, Intersection };
pub use crate::material::Material;
pub use crate::sphere::Sphere;
pub use crate::light::Light;
pub use crate::grid::Grid;

fn ray_direction(x: usize, y: usize, width: usize, height: usize, field_of_view: f64) -> Vec3f {
    let px: f64 = (2.0 * ((x as f64 + 0.5) / (width as f64)) - 1.0) * (field_of_view / 2.0).tan() * (width as f64 / height as f64);
    let py: f64 = (1.0 - 2.0 * ((y as f64 + 0.5) / (height as f64))) * (field_of_view / 2.0).tan();
    Vec3f::new(px, py, -1.0).normalized()
}

fn reflect(i: &Vec3f, n: &Vec3f) -> Vec3f {
    i.sub(&n.mult_scalar(2.0).mult_scalar(i.dot(&n)))
}

fn refract(i: &Vec3f, n: &Vec3f, refractive_index: f64) -> Vec3f {
    let cosi = -i.dot(&n).min(1.0).max(-1.0);
    let etai = if cosi < 0.0 { refractive_index } else { 1.0 };
    let etat = if cosi < 0.0 { 1.0 } else { refractive_index };
    let n = if cosi < 0.0 { n.mult_scalar(-1.0) } else { *n };
    let cosi = cosi.abs();
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3f::new(0.0, 0.0, 0.0)
    }
    else {
        i.mult_scalar(eta).add(&n.mult_scalar(eta * cosi - k.sqrt()))
    }
}

fn scene_intersect<'a>(origin: &Vec3f, direction: &Vec3f, objects: &Vec<&'a Intersectable>) -> Option<(Intersection, &'a Material)> {
    let mut material: Option<&Material> = None;
    let mut closest_intersection: Option<Intersection> = None;

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

fn cast_ray<'a>(origin: &Vec3f, direction: &Vec3f, objects: &'a Vec<&'a Intersectable>, lights: &Vec<Light>, bounces: isize) -> Option<Vec3f> {
    if bounces < 0 {
        return None;
    }

    let (intersection, material) = match scene_intersect(origin, direction, objects) {
        Some((i, m)) => (i, m),
        None => return None,
    };

    let reflect_direction = reflect(direction, &intersection.normal).normalized();
    let refract_direction = refract(direction, &intersection.normal, material.refractive_index).normalized();
    let reflect_origin: Vec3f = if reflect_direction.dot(&intersection.normal) < 0.0 {
        intersection.point.sub(&intersection.normal.mult_scalar(1e-3))
    }
    else {
        intersection.point.add(&intersection.normal.mult_scalar(1e-3))
    };
    let refract_origin: Vec3f = if refract_direction.dot(&intersection.normal) < 0.0 {
        intersection.point.sub(&intersection.normal.mult_scalar(1e-3))
    }
    else {
        intersection.point.add(&intersection.normal.mult_scalar(1e-3))
    };
    let reflect_colour = cast_ray(&reflect_origin, &reflect_direction, objects, lights, bounces - 1).unwrap_or(Vec3f::new(0.2, 0.7, 0.8));
    let refract_colour = cast_ray(&refract_origin, &refract_direction, objects, lights, bounces - 1).unwrap_or(Vec3f::new(0.2, 0.7, 0.8));

    let mut diffuse_light_intensity: f64 = 0.0;
    let mut specular_light_intensity: f64 = 0.0;
    for light in lights.iter() {
        let light_direction = light.position.sub(&intersection.point).normalized();
        let light_distance = light.position.sub(&intersection.point).length();

        let shadow_origin: Vec3f = if light_direction.dot(&intersection.normal) < 0.0 {
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
        .add(&Vec3f::new(1.0, 1.0, 1.0).mult_scalar(specular_light_intensity * material.albedo.1))
        .add(&reflect_colour.mult_scalar(material.albedo.2))
        .add(&refract_colour.mult_scalar(material.albedo.3))
    )
}

fn main() -> Result<(), io::Error> {
    let image_width = 1920;
    let image_height = 1080;
    let fov = std::f64::consts::FRAC_PI_2;

    let background = Vec3f::new(0.2, 0.7, 0.8);
    let ivory = Material { diffuse: Vec3f::new(0.4, 0.4, 0.3), albedo: (0.6, 0.3, 0.1, 0.0), specular_exponent: 50.0, refractive_index: 1.0 };
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };
    let mirror = Material { diffuse: Vec3f::new(1.0, 1.0, 1.0), albedo: (0.0, 10.0, 0.8, 0.0), specular_exponent: 1425.0, refractive_index: 1.0 };
    let glass = Material { diffuse: Vec3f::new(0.6, 0.7, 0.8), albedo: (0.0,  0.5, 0.1, 0.8), specular_exponent: 125.0, refractive_index: 1.5 };

    let objects = vec![
        Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0, &ivory),
        Sphere::new(Vec3f::new(-1.0, -1.5, -12.0), 2.0, &glass),
        Sphere::new(Vec3f::new(1.5, -0.5, -18.0), 3.0, &red_rubber),
        Sphere::new(Vec3f::new(7.0, 5.0, -18.0), 4.0, &mirror),
    ];

    let lights = vec![
        Light {
            position: Vec3f::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        Light {
            position: Vec3f::new(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        Light {
            position: Vec3f::new(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];

    let origin = Vec3f::new(0f64, 0f64, 0f64);
    let tiles: Vec<(usize, usize, usize, usize, Vec<Vec3f>)> = rayon::iter::split(Grid::new(image_width, image_height), Grid::split)
        .map(|g| {
            let width = g.x.end - g.x.start;
            let height = g.y.end - g.y.start;
            let mut framebuffer: Vec<Vec3f> = vec![Vec3f::default(); width * height];
            let objects = objects.iter().map(|o| o as &Intersectable).collect();

            for y in g.y.to_range() {
                for x in g.x.to_range() {
                    let dir = ray_direction(x, y, image_width, image_height, fov);

                    let i = x - g.x.start;
                    let j = y - g.y.start;
                    
                    framebuffer[i + (j * width)] = match cast_ray(&origin, &dir, &objects, &lights, 10) {
                        Some(c) => c.clone(),
                        None => background.clone(),
                    };
                }
            }

            (g.x.start, g.y.start, width, height, framebuffer)
        })
        .collect();
    
    let mut framebuffer: Vec<Vec3f> = vec![Vec3f::default(); image_width * image_height];
    for tile in tiles {
        let (tx, ty, tw, th, tfb) = tile;
        for tj in 0..th {
            for ti in 0..tw {
                assert_eq!(tw * th, tfb.len());
                let p = tfb[(tj * tw) + ti];
                framebuffer[((ty + tj) * image_width) + (tx + ti)] = p;
            }
        }
    }

    let file = fs::File::create("out.ppm")?;
    let mut buf = io::BufWriter::new(file);
    buf.write_fmt(format_args!("P6\n{} {}\n255\n", image_width, image_height))?;
    for i in 0..(image_width * image_height) {
        buf.write(&framebuffer[i].to_rgb())?;
    }

    println!("render saved to out.ppm");
    Ok(())
}

#[cfg(test)] mod tests;