use super::*;

#[test]
fn ray_should_intersect_sphere_direct() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(0.0, 0.0, -1.0);
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };
    let sphere: Sphere = Sphere::new(
        Vec3f::new(0.0, 0.0, -10.0),
        1.0,
        &red_rubber
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert_eq!(intersects.is_some(), true);
}

#[test]
fn ray_should_not_intersect_sphere_behind() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(0.0, 0.0, -1.0);
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };
    let sphere: Sphere = Sphere::new(
        Vec3f::new(0.0, 0.0, 10.0),
        1.0,
        &red_rubber
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert!(intersects.is_none());
}

#[test]
fn ray_should_intersect_sphere_centre_deflected() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(1.0, 0.0, -1.0).normalized();
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };
    let sphere: Sphere = Sphere::new(
        Vec3f::new(5.0, 0.0, -5.0),
        1.0,
        &red_rubber
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert!(intersects.is_some());
}

#[test]
fn ray_should_intersect_sphere_centre_deflected_2() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = Vec3f::new(1.0, 0.0, -1.0).normalized();
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };
    let sphere: Sphere = Sphere::new(
        Vec3f::new(4.0, 0.0, -5.0),
        1.0,
        &red_rubber
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert!(intersects.is_some());
}

#[test]
fn ray_should_miss() {
    let origin: Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    let direction: Vec3f = super::ray_direction(0, 0, 512, 512, std::f64::consts::FRAC_PI_2);
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };
    let sphere: Sphere = Sphere::new(
        Vec3f::new(0.0, 0.0, -10.0),
        1.0,
        &red_rubber
    );

    let intersects = sphere.ray_intersect(&origin, &direction);
    assert!(intersects.is_none());
}

#[test]
fn tiler_should_hit_all_tiles() {
    let w = 1920;
    let h = 1080;

    let sum = rayon::iter::split(Grid::new(w, h), Grid::split)
        .map(|g| {
            (g.x.end - g.x.start) * (g.y.end - g.y.start)
        })
        .sum();

    assert_eq!(w * h, sum);
}

#[bench]
fn simple_scene(b: &mut test::Bencher) {
    let screen_width = 1920;
    let screen_height = 1080;
    let fov = std::f64::consts::FRAC_PI_2;

    let background = Vec3f::new(0.0, 0.0, 0.0);
    let red_rubber = Material { diffuse: Vec3f::new(0.3, 0.1, 0.1), albedo: (0.9, 0.1, 0.0, 0.0), specular_exponent: 10.0, refractive_index: 1.0 };

    let objects = vec![
        Sphere::new(Vec3f::new(1.5, -0.5, -18.0), 3.0, &red_rubber),
    ];
    //let objects = objects.iter().map(|o| o as &Intersectable).collect();
    let lights = vec![
        Light {
            position: Vec3f::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
    ];
    let origin = Vec3f::new(0f64, 0f64, 0f64);
    
    b.iter(|| {
        let tiles: Vec<(usize, usize, usize, usize, Vec<Vec3f>)> = rayon::iter::split(Grid::new(screen_width, screen_height), Grid::split)
            .map(|g| {
                let width = g.x.end - g.x.start;
                let height = g.y.end - g.y.start;
                let mut framebuffer: Vec<Vec3f> = vec![Vec3f::default(); width * height];
                let objects = objects.iter().map(|o| o as &Intersectable).collect();

                for y in g.y.to_range() {
                    for x in g.x.to_range() {
                        let dir = ray_direction(x, y, screen_width, screen_height, fov);

                        let i = x - g.x.start;
                        let j = y - g.y.start;
                        
                        framebuffer[i + (j * width)] = match cast_ray(&origin, &dir, &objects, &lights, 4) {
                            Some(c) => c.clone(),
                            None => background.clone(),
                        };
                    }
                }

                (g.x.start, g.y.start, width, height, framebuffer)
            })
            .collect();
        
        let mut framebuffer: Vec<Vec3f> = vec![Vec3f::default(); screen_width * screen_height];
        for tile in tiles {
            let (tx, ty, tw, th, tfb) = tile;
            for tj in 0..th {
                for ti in 0..tw {
                    assert_eq!(tw * th, tfb.len());
                    let p = tfb[(tj * tw) + ti];
                    framebuffer[((ty + tj) * screen_width) + (tx + ti)] = p;
                }
            }
        }
    });
}