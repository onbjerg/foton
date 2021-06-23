use super::hitable::{Hit, Hitable};
use core::ray::Ray;
use core::vec3::Vec3;
use materials::Scatterable;

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<dyn Scatterable>,
}

impl Sphere {
    pub fn new<T: Scatterable + 'static>(centre: Vec3, radius: f32, material: T) -> Sphere {
        Sphere {
            centre,
            radius,
            material: Box::new(material),
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            [
                (-b - (b * b - a * c).sqrt()) / a,
                (-b + (b * b - a * c).sqrt()) / a,
            ]
            .iter()
            .find(|solution| **solution < t_max && **solution > t_min)
            .map(|solution| Hit {
                t: *solution,
                point: ray.point_at_parameter(*solution),
                normal: (ray.point_at_parameter(*solution) - self.centre) / self.radius,
                material: &self.material,
            })
        } else {
            None
        }
    }
}
