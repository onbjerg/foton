use super::{random_point_in_sphere, Scatter, Scatterable};
use core::ray::Ray;
use core::vec3::Vec3;
use scenery::hitable::Hit;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.normal + random_point_in_sphere();

        Some(Scatter {
            ray: Ray::new(hit.point, target),
            attenuation: self.albedo,
        })
    }
}
