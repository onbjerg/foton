extern crate rand;

use ::core::ray::Ray;
use ::core::vec3::Vec3;
use ::scenery::hitable::Hit;
use self::lambertian::Lambertian;
use self::metal::Metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal)
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(&ray, &hit),
            Material::Metal(ref inner) => inner.scatter(&ray, &hit)
        }
    }
}

pub fn random_point_in_sphere() -> Vec3 {
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

pub mod lambertian;
pub mod metal;
