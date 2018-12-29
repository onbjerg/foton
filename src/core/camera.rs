use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal_offset: Vec3,
    vertical_offset: Vec3,
    origin: Vec3,
    lens_radius: f32,
    ortho: [Vec3; 3]
}

impl Camera {
    // `fov` is vertical FOV, in degrees, from top to bottom
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        fov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        // Convert to radians
        let theta = fov * ::std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // Compute orthonormal basis
        let w = (look_from - look_at).normalize();
        let u = (view_up.cross(&w)).normalize();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: look_from - half_width * u * focus_distance - half_height * v * focus_distance - w * focus_distance,
            horizontal_offset: 2.0 * half_width * u * focus_distance,
            vertical_offset: 2.0 * half_height * v * focus_distance,
            origin: look_from,
            ortho: [w, u ,v],
            lens_radius
        }
    }

    pub fn cast_ray(&self, s: f32, t: f32) -> Ray {
        let random_point = self.lens_radius * random_point_in_unit_disk();
        let offset = self.ortho[1] * random_point.x + self.ortho[2] * random_point.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal_offset + t * self.vertical_offset - self.origin - offset
        )
    }
}

fn random_point_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random(), rand::random(), 0.0) - Vec3::new(1.0, 1.0, 0.0);

        if p.dot(&p) < 1.0 {
            return p
        }
    }
}
