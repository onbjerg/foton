use ::core::vec3::Vec3;
use ::core::ray::Ray;
use ::materials::Scatterable;
use super::hitable::{Hit, Hitable};

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<dyn Scatterable>
}

impl Sphere {
    pub fn new (centre: Vec3, radius: f32, material: Box<dyn Scatterable>) -> Sphere {
        Sphere {
            centre,
            radius,
            material
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
            // TODO: Somehow clean up cus u suck
            let solution = (-b - (b * b - a * c).sqrt()) / a;
            if solution < t_max && solution > t_min {
                return Some(Hit {
                    t: solution,
                    point: ray.point_at_parameter(solution),
                    normal: (ray.point_at_parameter(solution) - self.centre) / self.radius,
                    material: &self.material
                });
            }

            let solution = (-b + (b * b - a * c).sqrt()) / a;
            if solution < t_max && solution > t_min {
                return Some(Hit {
                    t: solution,
                    point: ray.point_at_parameter(solution),
                    normal: (ray.point_at_parameter(solution) - self.centre) / self.radius,
                    material: &self.material
                });
            }
        }

        None
    }
}