extern crate rand;

use ::core::ray::Ray;
use ::core::vec3::Vec3;
use ::scenery::hitable::Hit;
use super::{Scatter, Scatterable};

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(normal) * *normal
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = reflect(&ray.direction.normalize(), &hit.normal);

        if target.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                ray: Ray::new(hit.point, target),
                attenuation: self.albedo
            })
        } else {
            None
        }
    }
}
