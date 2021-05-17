pub mod sphere;

use crate::cgmath::{
    aabb::AxisAlignedBoundingBox, normal::NormalizedVec3, point3::Point3, ray::Ray,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct ShapeHit {
    pub point: Point3,
    pub normal: NormalizedVec3,
    pub t: f32,
    pub is_front_face: bool,
}

pub trait HittableShape: Sync + Send + Debug {
    fn aabb(&self) -> AxisAlignedBoundingBox;

    fn hit(
        &self,
        ray: &Ray,
        near: f32,
        far: f32,
    ) -> Option<ShapeHit>;
}

crate::define_variant!(ShapeDef: Clone {
    UnitSphere(sphere::UnitSphere)
});

impl HittableShape for ShapeDef {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        match self {
            ShapeDef::UnitSphere(s) => s.aabb(),
        }
    }

    fn hit(
        &self,
        ray: &Ray,
        near: f32,
        far: f32,
    ) -> Option<ShapeHit> {
        match self {
            ShapeDef::UnitSphere(s) => s.hit(ray, near, far),
        }
    }
}
