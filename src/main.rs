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

fn random_point_in_sphere() -> Vec3 {
    let mut point = Vec3::new(0.0, 0.0, 0.0);
    while {
        // do
        point = Vec3::new(rand::random(), rand::random(), rand::random()) - Vec3::new(1.0, 1.0, 1.0);

        // while
        point.squared_length() >= 1.0

        // yes thanks rust, very beautiful do-while loops
    } {};

    point
}

fn color_for_ray(ray: &Ray, scene: &Scene) -> Vec3 {
    let base_color = Vec3::new(0.0, 0.0, 1.0);
    let unit_vector = Vec3::new(1.0, 1.0, 1.0);
    let unit_direction = ray.direction.normalize();

    let t = 0.5 * (unit_direction.y + 1.0);
    let c = (1.0 - t) * unit_vector + t * base_color;

    // Note that `t_min` is not exactly 0.0 to solve shadow acne
    match scene.hit(&ray, 0.001, std::f32::MAX) {
        Some(hit) => 0.5 * color_for_ray(&Ray::new(
            hit.point,
            hit.normal + random_point_in_sphere()
        ), &scene),

        // Note that square rooting is for simple gamma correction
        None => 255.0 * Vec3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt())
    }
}

fn main() {
    // Constants
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;
    const SAMPLES: usize = 100;

    // Image data buffer
    let mut data = [RGBA(0, 0, 0, 0); WIDTH * HEIGHT];

    // Create camera
    let camera = Camera::new();

    // Create scene
    let mut scene = Scene::new();
    scene.add_sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.add_sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

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

                    color_at_point + color_for_ray(&ray, &scene)
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
