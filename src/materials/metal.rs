use super::{random_point_in_sphere, Scatter, Scatterable};
use core::ray::Ray;
use core::vec3::Vec3;
use scenery::hitable::Hit;

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(normal) * *normal
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = reflect(&ray.direction.normalize(), &hit.normal);

        if target.dot(&hit.normal) > 0.0 {
            Some(Scatter {
                ray: Ray::new(hit.point, target + self.fuzz * random_point_in_sphere()),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
