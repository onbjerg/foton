extern crate rand;

use ::core::ray::Ray;
use ::core::vec3::Vec3;
use ::scenery::hitable::Hit;
use super::{Scatter, Scatterable};

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3
}

fn random_point_in_sphere() -> Vec3 {
    let mut point: Vec3;
    while {
        // do
        point = Vec3::new(rand::random(), rand::random(), rand::random()) - Vec3::new(1.0, 1.0, 1.0);

        // while
        point.squared_length() >= 1.0

        // yes thanks rust, very beautiful do-while loops
    } {};

    point
}

impl Scatterable for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.normal + random_point_in_sphere();

        Some(Scatter {
            ray: Ray::new(hit.point, target),
            attenuation: self.albedo
        })
    }
}
