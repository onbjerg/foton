extern crate lodepng;
extern crate rand;

mod core;
mod scenery;

use core::vec3::Vec3;
use core::ray::Ray;
use core::rgba::RGBA;
use core::camera::Camera;

use scenery::hitable::Hitable;
use scenery::sphere::Sphere;
use scenery::scene::Scene;

fn color_for_ray(ray: &Ray) -> Vec3 {
    let unit_vector = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0
    };
    let base_color = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0
    };
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    let c = (1f32 - t) * unit_vector + t * base_color;

    let mut scene = Scene::new();
    scene.add_sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.add_sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    match scene.hit(&ray, 0.0, 100.0) {
        Some(hit) => 0.5 * Vec3::new(
            255.0 * (hit.normal.x + 1.0),
            255.0 * (hit.normal.y + 1.0),
            255.0 * (hit.normal.z + 1.0)
        ),
        None => 255f32 * c
    }
}

fn main() {
    // Constants
    const WIDTH: usize = 600;
    const HEIGHT: usize = 300;
    const SAMPLES: usize = 100;

    // Image data buffer
    let mut data = [RGBA(0, 0, 0, 0); WIDTH * HEIGHT];

    // Create camera
    let camera = Camera::new();

    // Render scene
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Take `n` samples for this point and average the color
            let color_at_point = (0..SAMPLES).fold(
                Vec3::new(0.0, 0.0, 0.0),
                |color_at_point, _| {
                    // Get ray origin from current position
                    // with a fuzzy factor on top for AA sampling
                    let u: f32 = (x as f32 + rand::random::<f32>()) / WIDTH as f32;
                    let v: f32 = (y as f32 + rand::random::<f32>()) / HEIGHT as f32;

                    let ray = camera.cast_ray(u, v);

                    color_at_point + color_for_ray(&ray)
                }
            ) / (SAMPLES as f32);

            // Write data to image buffer
            data[x + (HEIGHT - 1 - y) * WIDTH] = RGBA(
                color_at_point.x as u8,
                color_at_point.y as u8,
                color_at_point.z as u8,
                255
            );
        }
    }

    // Output image
    lodepng::encode32_file("out.png", &data, WIDTH, HEIGHT);
}
