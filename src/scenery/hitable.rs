use ::core::vec3::Vec3;
use ::core::ray::Ray;
use ::materials::Scatterable;

pub struct Hit<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Scatterable>
}

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
