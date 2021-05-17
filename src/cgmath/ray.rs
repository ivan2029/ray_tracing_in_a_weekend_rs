use crate::cgmath::{vec3::Vec3, point3::Point3, transform::Transform};

/*
 */
#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(
        origin: Point3,
        direction: Vec3,
    ) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(
        &self,
        t: f32,
    ) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn apply_transform(
        &self,
        t: &Transform,
    ) -> Ray {
        Ray {
            origin: self.origin.apply_transform(t),
            direction: self.direction.apply_transform(t),
        }
    }
}
