use ::core::ray::Ray;
use super::hitable::{Hit, Hitable};

pub struct Scene {
    scenery: Vec<Box<dyn Hitable>>
}

impl Scene {
    pub fn new() -> Scene {
        let scenery: Vec<Box<dyn Hitable>> = Vec::new();
        Scene {
            scenery
        }
    }

    pub fn add_sphere(&mut self, obj: Box<dyn Hitable>) {
        self.scenery.push(obj);
    }
}

impl Hitable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.scenery.iter().fold(
            None,
            |hit, s|
                s.hit(ray, t_min, hit.as_ref().map_or(t_max, |h| h.t)).or(hit)
        )
    }
}
