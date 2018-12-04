use ::core::vec3::Vec3;
use ::core::ray::Ray;

pub struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
