use crate::{
    cgmath::{angle::Radians, normal::NormalizedVec3, point3::Point3, ray::Ray, vec3::Vec3},
    raytracer::camera::Camera,
};

/*
 */
#[derive(Debug)]
pub struct PerspectiveCamera {
    eye: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    // coordinate system
    u: NormalizedVec3,
    v: NormalizedVec3,
    w: NormalizedVec3,
    //
    lens_radius: f32,
}

impl PerspectiveCamera {
    pub fn new(
        eye: Point3,
        target: Point3,
        up: Vec3,
        vertical_fov: Radians,
        aspect_ratio: f32,
        aperture: f32,
        focal_distance: f32,
    ) -> PerspectiveCamera {
        let h = (vertical_fov.0 * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (eye - target).normalized();
        let u = Vec3::cross(up, w.to_vec3()).normalized();
        let v = Vec3::cross(w.to_vec3(), u.to_vec3()).normalized();

        let horizontal = (focal_distance * viewport_width) * u.to_vec3();
        let vertical = (focal_distance * viewport_height) * v.to_vec3();
        let lower_left_corner =
            eye - (horizontal * 0.5 + vertical * 0.5 + focal_distance * w.to_vec3());

        let lens_radius = aperture * 0.5;

        PerspectiveCamera {
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
}

impl Camera for PerspectiveCamera {
    fn ray_at(
        &self,
        s: f32,
        t: f32,
    ) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u.to_vec3() * rd.x() + self.v.to_vec3() * rd.y();

        let origin = self.eye + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;

        Ray::new(origin, direction)
    }
}
