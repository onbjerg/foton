use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal_offset: Vec3,
    vertical_offset: Vec3,
    origin: Vec3
}

impl Camera {
    // `fov` is vertical FOV, in degrees, from top to bottom
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        fov: f32,
        aspect: f32
    ) -> Camera {
        // Convert to radians
        let theta = fov * ::std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // Compute orthonormal basis
        let w = (look_from - look_at).normalize();
        let u = (view_up.cross(&w)).normalize();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal_offset: 2.0 * half_width * u,
            vertical_offset: 2.0 * half_height * v,
            origin: look_from
        }
    }

    pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal_offset + v * self.vertical_offset - self.origin
        )
    }
}
