use crate::cgmath::*;
use crate::raytracer::ray::*;

//
//
//
#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl HittableShape for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        near: f32,
        far: f32,
    ) -> Option<ShapeHit> {
        let center_origin = *ray.origin() - self.center;

        let a = ray.direction().norm_squared();
        let half_b = Vec3::dot(center_origin, *ray.direction());
        let c = center_origin.norm_squared() - self.radius * self.radius;

        let d = half_b * half_b - a * c;

        if d < 0.0 {
            return None;
        }

        let d = d.sqrt();

        let t1 = (-half_b - d) / a;
        let t2 = (-half_b + d) / a;
        let t = match (t1 > 0.0, t2 > 0.0) {
            (true, true) => f32::min(t1, t2),
            (true, false) => t1,
            (false, true) => t2,
            _ => return None,
        };

        if !(near < t && t < far) {
            return None;
        }

        //
        let point = ray.at(t);
        let normal = self.radius.signum() * (point - self.center).normalized();

        let is_front_face = Vec3::dot(normal, *ray.direction()) < 0.0;

        let normal = if is_front_face { normal } else { -normal };

        Some(ShapeHit {
            point,
            normal,
            t,
            is_front_face,
        })
    }
}
