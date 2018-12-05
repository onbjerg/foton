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

pub mod lambertian;
pub mod metal;
