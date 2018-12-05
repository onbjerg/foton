extern crate lodepng;
extern crate rand;

mod core;
mod scenery;
mod materials;

use core::vec3::Vec3;
use core::ray::Ray;
use core::camera::Camera;

use scenery::hitable::Hitable;
use scenery::sphere::Sphere;
use scenery::scene::Scene;

use materials::{Material, Scatterable};
use materials::lambertian::Lambertian;
use materials::metal::Metal;

fn color_for_ray(ray: &Ray, scene: &Scene, depth: u8) -> Vec3 {
    let base_color = Vec3::new(0.0, 0.0, 1.0);
    let unit_vector = Vec3::new(1.0, 1.0, 1.0);
    let unit_direction = ray.direction.normalize();

    let t = 0.5 * (unit_direction.y + 1.0);
    let c = (1.0 - t) * unit_vector + t * base_color;

    // Note that `t_min` is not exactly 0.0 to solve shadow acne
    match scene.hit(&ray, 0.001, std::f32::MAX) {
        // TODO: Clean the scatter handling up a bit
        Some(hit) => match hit.material.scatter(&ray, &hit) {
            Some(scatter) => {
                if depth < 50 {
                    return scatter.attenuation * color_for_ray(&scatter.ray, &scene, depth + 1);
                }

                Vec3::new(0.0, 0.0, 0.0)
            },
            _ => Vec3::new(0.0, 0.0, 0.0)
        },
        None => c
    }
}

fn main() {
    // Constants
    const WIDTH: usize = 1200;
    const HEIGHT: usize = 600;
    const SAMPLES: usize = 100;

    // Image data buffer
    let mut data = [(0u8, 0u8, 0u8); WIDTH * HEIGHT];

    // Create camera
    let camera = Camera::new();

    // Create scene
    let mut scene = Scene::new();
    scene.add_sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.3, 0.3)
    })));
    scene.add_sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0)
    })));
    scene.add_sphere(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.3
    })));
    scene.add_sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Metal(Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 1.0
    })));

    // Render scene
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Take `n` samples for this point and average the color
            let color_at_point = (0..SAMPLES).fold(
                Vec3::new(0.0, 0.0, 0.0),
                |color_at_point, _| {
                    // Get ray origin from current position with a fuzzy factor on top for AA sampling
                    let u: f32 = (x as f32 + rand::random::<f32>()) / WIDTH as f32;
                    let v: f32 = (y as f32 + rand::random::<f32>()) / HEIGHT as f32;

                    let ray = camera.cast_ray(u, v);

                    color_at_point + color_for_ray(&ray, &scene, 0)
                }
            ) / (SAMPLES as f32);

            // Write data to image buffer
            // Note that the square rooting is gamma correction
            data[x + (HEIGHT - 1 - y) * WIDTH] = (
                (255.0 * color_at_point.x.sqrt()) as u8,
                (255.0 * color_at_point.y.sqrt()) as u8,
                (255.0 * color_at_point.z.sqrt()) as u8
            );
        }
    }

    // Output image
    match lodepng::encode24_file("out.png", &data, WIDTH, HEIGHT) {
        Ok(_) => println!("Rendered scene to out.png"),
        Err(e) => println!("{}", e)
    }
}
