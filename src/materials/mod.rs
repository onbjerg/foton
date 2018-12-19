extern crate rand;

use ::core::ray::Ray;
use ::core::vec3::Vec3;
use ::scenery::hitable::Hit;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3
}

pub trait Scatterable: Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub fn random_point_in_sphere() -> Vec3 {
    let mut point: Vec3;
    while {
        // do
        point = 2.0 * Vec3::new(rand::random(), rand::random(), rand::random()) - Vec3::new(1.0, 1.0, 1.0);

        // while
        point.squared_length() >= 1.0

        // yes thanks rust, very beautiful do-while loops
    } {};

    point
}

pub mod lambertian;
pub mod metal;
pub mod diaelectric;
