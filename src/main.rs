extern crate lodepng;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;

mod core;
mod scenery;
mod materials;

use core::vec3::Vec3;
use core::ray::Ray;
use core::camera::Camera;

use scenery::hitable::Hitable;
use scenery::sphere::Sphere;
use scenery::scene::Scene;

use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::diaelectric::Diaelectric;

fn color_for_ray(ray: &Ray, scene: &Scene, depth: u8) -> Vec3 {
    // Note that `t_min` is not exactly 0.0 to solve shadow acne
    match scene.hit(&ray, 0.001, std::f32::MAX) {
        Some(ref hit) if depth < 50 => hit.material.scatter(&ray, hit).map_or(
            Vec3::new(0.0, 0.0, 0.0),
            |scatter| scatter.attenuation * color_for_ray(&scatter.ray, &scene, depth + 1)
        ),
        Some(_) if depth > 50 => Vec3::new(0.0, 0.0, 0.0),
        _ => {
            let base_color = Vec3::new(0.5, 0.7, 1.0);
            let unit_vector = Vec3::new(1.0, 1.0, 1.0);
            let unit_direction = ray.direction.normalize();

            let t = 0.5 * (unit_direction.y + 1.0);

            (1.0 - t) * unit_vector + t * base_color
        }
    }
}

fn make_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add_object(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5)
        }
    ));

    for i in -11..11 {
        for j in -11..11 {
            let material_chooser: f32 = rand::random();
            let sphere_center = Vec3::new(
                (i as f32) + 0.9 * rand::random::<f32>(),
                0.2,
                (j as f32) + 0.9 * rand::random::<f32>()
            );
            if (sphere_center - Vec3::new(0.4, 0.2, 0.0)).length() > 0.9 {
                if material_chooser < 0.8 {
                    scene.add_object(Sphere::new(
                    sphere_center,
                    0.2,
                    Lambertian {
                        albedo: Vec3::new(rand::random(), rand::random(), rand::random())
                    }));
                } else if material_chooser < 0.95 {
                    scene.add_object(Sphere::new(
                    sphere_center,
                    0.2,
                    Metal {
                        albedo: 0.5 * Vec3::new(
                            1.0 + rand::random::<f32>(),
                            1.0 + rand::random::<f32>(),
                            1.0 + rand::random::<f32>()
                        ),
                        fuzz: rand::random()
                    }));
                } else {
                    scene.add_object(Sphere::new(
                    sphere_center,
                    0.2,
                    Diaelectric {
                        refraction_index: rand::random()
                    }));
                };
            }
        }
    }

    // Add 3 big center spheres
    scene.add_object(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Diaelectric {
            refraction_index: 1.0
        }
    ));
    scene.add_object(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian {
            albedo: Vec3::new(0.4, 0.4, 0.2)
        }
    ));
    scene.add_object(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0
        }
    ));

    scene
}

fn main() {
    // Constants
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    const SAMPLES: usize = 100;

    // Image data buffer
    let mut data = [(0u8, 0u8, 0u8); WIDTH * HEIGHT];

    // Create camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (WIDTH as f32) / (HEIGHT as f32),
        0.1,
        10.0
    );

    // Create scene
    let scene = make_scene();

    // Render scene
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Take `n` samples for this point and average the color
            let color_at_point = (0..SAMPLES).into_par_iter().map(
                |_| {
                    // Get ray origin from current position with a fuzzy factor on top for AA sampling
                    let u: f32 = (x as f32 + rand::random::<f32>()) / WIDTH as f32;
                    let v: f32 = (y as f32 + rand::random::<f32>()) / HEIGHT as f32;

                    color_for_ray(&camera.cast_ray(u, v), &scene, 0)
                }
            ).reduce(
                || Vec3::new(0.0, 0.0, 0.0),
                |color_at_point, color| color_at_point + color
            ) / (SAMPLES as f32);

            // Write data to image buffer
            // Note that the square rooting is gamma correction
            data[x + (HEIGHT - 1 - y) * WIDTH] = (
                (255.99 * color_at_point.x.sqrt()) as u8,
                (255.99 * color_at_point.y.sqrt()) as u8,
                (255.99 * color_at_point.z.sqrt()) as u8
            );
        }
    }

    // Output image
    match lodepng::encode24_file("out.png", &data, WIDTH, HEIGHT) {
        Ok(_) => println!("Rendered scene to out.png"),
        Err(e) => println!("{}", e)
    }
}
