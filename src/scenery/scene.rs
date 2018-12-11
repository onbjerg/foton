use ::core::ray::Ray;
use super::hitable::{Hit, Hitable};
use super::sphere::Sphere;

pub struct Scene {
    spheres: Vec<Sphere>
}

impl Scene {
    pub fn new() -> Scene {
        let spheres: Vec<Sphere> = Vec::new();
        Scene {
            spheres
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

impl Hitable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.spheres.iter().fold(
            None,
            |hit, s| s.hit(ray, t_min, hit.t).or(hit)
        })
    }
}
