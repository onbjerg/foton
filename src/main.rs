extern crate lodepng;
extern crate rand;

use rand::Rng;

mod core;

use core::vec3::Vec3;
use core::ray::Ray;
use core::rgba::RGBA;

fn hit_sphere(sphere_center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - sphere_center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color_for_ray(ray: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let normal = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vec3::new(
            255.0 * (normal.x + 1.0),
            255.0 * (normal.y + 1.0),
            255.0 * (normal.z + 1.0))
    }

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

    255f32 * c
}

fn main() {
    // Constants
    const WIDTH: usize = 1200;
    const HEIGHT: usize = 600;
    const SAMPLES: usize = 100;

    // The world is from (-2.0, -1.0, -1.0) to (2.0, 1.0, -1.0)
    // We start at the lower left corner and work our way up using offset vectors
    const LOWER_LEFT_CORNER: Vec3 = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0
    };
    const HORIZONTAL_OFFSET: Vec3 = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0
    };
    const VERTICAL_OFFSET: Vec3 = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0
    };

    // The camera is positioned at (0.0, 0.0, 0.0)
    const CAMERA_ORIGIN: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

    // Image data buffer
    let mut data = [RGBA(0, 0, 0, 0); WIDTH * HEIGHT];

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

                    let ray = Ray::new(
                        CAMERA_ORIGIN,
                        LOWER_LEFT_CORNER + u * HORIZONTAL_OFFSET + v * VERTICAL_OFFSET
                    );

                    color_at_point + color_for_ray(&ray)
                }
            ) / (SAMPLES as f32);

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
