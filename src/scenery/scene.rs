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

    pub fn add_object<T: Hitable + 'static>(&mut self, obj: T) {
        self.scenery.push(Box::new(obj));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenery::sphere::Sphere;
    use crate::core::vec3::Vec3;
    use crate::materials::lambertian::Lambertian;

    #[test]
    fn empty_scene_has_no_hits() {
        let scene = Scene::new();
        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0)
        );

        assert!(scene.hit(&ray, 0.0, 100.0).is_none());
    }

    #[test]
    fn no_hit() {
        let mut scene = Scene::new();
        scene.add_sphere(Box::new(Sphere::new(
            Vec3::new(1.0, 1.0, -1.0),
            0.5,
            Box::new(Lambertian {
                albedo: Vec3::new(0.0, 0.0, 0.0)
            })
        )));

        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0)
        );

        assert!(scene.hit(&ray, 0.0, 100.0).is_none());
    }

    #[test]
    fn hit() {
        let mut scene = Scene::new();
        scene.add_sphere(Box::new(Sphere::new(
            Vec3::new(1.0, 1.0, -1.0),
            0.5,
            Box::new(Lambertian {
                albedo: Vec3::new(0.0, 0.0, 0.0)
            })
        )));

        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, -1.0)
        );

        assert!(scene.hit(&ray, 0.0, 100.0).is_some());
    }

    #[test]
    fn hit_closest_object() {
        let mut scene = Scene::new();
        scene.add_sphere(Box::new(Sphere::new(
            Vec3::new(1.0, 1.0, -1.0),
            0.5,
            Box::new(Lambertian {
                albedo: Vec3::new(0.0, 0.0, 0.0)
            })
        )));
        scene.add_sphere(Box::new(Sphere::new(
            Vec3::new(1.0, 1.0, -1.5),
            0.5,
            Box::new(Lambertian {
                albedo: Vec3::new(0.0, 0.0, 0.0)
            })
        )));

        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, -2.0)
        );

        assert!(scene.hit(&ray, 0.0, 100.0).is_some());
        assert_eq!(scene.hit(&ray, 0.0, 100.0).unwrap().t, 0.6666667);
    }
}