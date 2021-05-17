use crate::cgmath::{transform::Transform, vec3::Vec3};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
    ) -> Point3 {
        assert!(!x.is_nan());
        assert!(!y.is_nan());
        assert!(!z.is_nan());
        Point3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn set_x(
        &mut self,
        value: f32,
    ) {
        assert!(!value.is_nan());
        self.x = value;
    }

    pub fn set_y(
        &mut self,
        value: f32,
    ) {
        assert!(!value.is_nan());
        self.y = value;
    }

    pub fn set_z(
        &mut self,
        value: f32,
    ) {
        assert!(!value.is_nan());
        self.z = value;
    }

    pub fn apply_transform(
        &self,
        t: &Transform,
    ) -> Point3 {
        let m = t.forward();
        Point3::new(
            m[0][0] * self.x + m[0][1] * self.y + m[0][2] * self.z + m[0][3],
            m[1][0] * self.x + m[1][1] * self.y + m[1][2] * self.z + m[1][3],
            m[2][0] * self.x + m[2][1] * self.y + m[2][2] * self.z + m[2][3],
        )
    }

    pub fn pos(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn offset_from(
        &self,
        other: Point3,
    ) -> Vec3 {
        other - *self
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;
    fn add(
        self,
        offset: Vec3,
    ) -> Point3 {
        Point3::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

impl Add<Point3> for Vec3 {
    type Output = Point3;
    fn add(
        self,
        point: Point3,
    ) -> Point3 {
        point + self
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;
    fn sub(
        self,
        other: Point3,
    ) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;
    fn sub(
        self,
        offset: Vec3,
    ) -> Point3 {
        Point3::new(
            self.x - offset.x(),
            self.y - offset.y(),
            self.z - offset.z(),
        )
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Point3 {
        Point3 {
            x: v.x(),
            y: v.y(),
            z: v.z(),
        }
    }
}
