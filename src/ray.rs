use crate::cgmath::Vec3;

use std::fmt::Debug;

//
//
//

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        let direction = direction.normalized();
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

//
//
//
#[derive(Debug, Clone, Default)]
pub struct ShapeHit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
}

pub trait HittableShape: Sync + Send + Debug {
    fn hit(&self, ray: &Ray, near: f32, far: f32) -> Option<ShapeHit>;
}
