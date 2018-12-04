use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal_offset: Vec3,
    vertical_offset: Vec3,
    origin: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal_offset: Vec3::new(4.0, 0.0, 0.0),
            vertical_offset: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal_offset + v * self.vertical_offset
        )
    }
}
