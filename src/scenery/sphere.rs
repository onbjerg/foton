use ::core::vec3::Vec3;
use ::core::ray::Ray;
use super::hitable::{Hit, Hitable};

pub struct Sphere {
    centre: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new (centre: Vec3, radius: f32) -> Sphere {
        Sphere {
            centre,
            radius
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
            let solution = (-b - (b * b - a * c).sqrt()) / a;
            if solution < t_max && solution > t_min {
                return Some(Hit {
                    t: solution,
                    point: ray.point_at_parameter(solution),
                    normal: (ray.point_at_parameter(solution) - self.centre) / self.radius
                });
            }

            let solution = (-b + (b * b - a * c).sqrt()) / a;
            if solution < t_max && solution > t_min {
                return Some(Hit {
                    t: solution,
                    point: ray.point_at_parameter(solution),
                    normal: (ray.point_at_parameter(solution) - self.centre) / self.radius
                });
            }
        }

        None
    }
}