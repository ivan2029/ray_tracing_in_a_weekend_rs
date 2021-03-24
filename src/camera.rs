use crate::cgmath::*;
use crate::ray::*;

#[derive(Debug)]
pub struct Camera {
    eye: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        eye: Vec3,
        target: Vec3,
        up: Vec3,
        vertical_fov: Radians,
        aspect_ratio: f32,
        aperture: f32,
        focal_distance: f32,
    ) -> Camera {
        let h = (vertical_fov.0 * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (eye - target).normalized();
        let u = Vec3::cross(up, w).normalized();
        let v = Vec3::cross(w, u);

        let horizontal = (focal_distance * viewport_width) * u;
        let vertical = (focal_distance * viewport_height) * v;
        let lower_left_corner = eye - (horizontal * 0.5 + vertical * 0.5 + focal_distance * w);

        let lens_radius = aperture * 0.5;

        Camera {
            eye,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn ray_at(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.eye + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;

        Ray::new(origin, direction)
    }
}
