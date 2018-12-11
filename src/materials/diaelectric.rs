extern crate rand;

use ::core::ray::Ray;
use ::core::vec3::Vec3;
use ::scenery::hitable::Hit;
use super::{Scatter, Scatterable};

#[derive(Clone, Copy)]
pub struct Diaelectric {
    pub refraction_index: f32
}

/// Calculate the probability of reflection for a material at an angle
fn shlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

/// Reflect a vector from a normal
fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(normal) * *normal
}

/// Refract a vector using Snell's law
fn refract (v: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let unit_vector = v.normalize();
    let dt = unit_vector.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (unit_vector - *normal * dt) - *normal * discriminant.sqrt())
    } else {
        None
    }
}

impl Scatterable for Diaelectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_index = self.refraction_index;

        let params: (Vec3, f32, f32) = {
            if ray.direction.dot(&hit.normal) > 0.0 {
                (
                    -1.0 * hit.normal,
                    refraction_index,
                    refraction_index * ray.direction.dot(&hit.normal) / ray.direction.length()
                )
            } else {
                (
                    hit.normal,
                    1.0 / refraction_index,
                    (-1.0 * ray.direction.dot(&hit.normal)) / ray.direction.length()
                )
            }
        };

        let ray_direction = match refract(
            &ray.direction,
            &params.0,
            params.1
        ) {
            Some(refracted) if rand::random::<f32>() > shlick(params.2, refraction_index) => refracted,
            _ => reflect(&ray.direction, &hit.normal)
        };

        Some(Scatter {
            ray: Ray::new(hit.point, ray_direction),
            attenuation
        })
    }
}
