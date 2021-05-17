/*
 */
use crate::{
    cgmath::{aabb::AxisAlignedBoundingBox, ray::Ray, vec3::Vec3},
    raytracer::shape::{HittableShape, ShapeHit},
};
#[derive(Debug, Clone)]
pub struct UnitSphere;

impl HittableShape for UnitSphere {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new(-1.0..1.0, -1.0..1.0, -1.0..1.0)
    }

    fn hit(
        &self,
        ray: &Ray,
        near: f32,
        far: f32,
    ) -> Option<ShapeHit> {
        let ray_origin_norm_squared = ray.origin().pos().norm_squared();

        let b = Vec3::dot(ray.origin().into(), ray.direction()) * 2.0;
        let c = ray_origin_norm_squared - 1.0;

        let disc = b * b - 4.0 * c;

        if disc < 0.0 {
            return None;
        }

        let disc = disc.sqrt();

        let t1 = 0.5 * (-b + disc);
        let t2 = 0.5 * (-b - disc);

        let t = match (t1 > 0.0, t2 > 0.0) {
            (true, true) => t1.min(t2),
            (true, false) => t1,
            (false, true) => t2,
            _ => return None,
        };

        if t < near || t > far {
            return None;
        };

        let point = ray.at(t);

        let normal = point.pos().normalized();

        let is_front_face = ray_origin_norm_squared > 1.0;

        Some(ShapeHit {
            point,
            normal,
            t,
            is_front_face,
        })
    }
}
