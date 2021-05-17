use crate::cgmath::{transform::Transform, vec3::Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct NormalizedVec3 {
    vec: Vec3,
}

impl NormalizedVec3 {
    pub fn from_vec(vec: Vec3) -> NormalizedVec3 {
        assert!(!vec.is_near_zero());
        NormalizedVec3 {
            vec: vec / vec.norm(),
        }
    }

    pub fn from_xyz(
        x: f32,
        y: f32,
        z: f32,
    ) -> NormalizedVec3 {
        NormalizedVec3::from_vec(Vec3::new(x, y, z))
    }

    pub fn to_vec3(self) -> Vec3 {
        self.vec
    }

    pub fn x(&self) -> f32 {
        self.vec.x()
    }

    pub fn y(&self) -> f32 {
        self.vec.y()
    }

    pub fn z(&self) -> f32 {
        self.vec.z()
    }

    pub fn apply_transform(
        &self,
        t: &Transform,
    ) -> NormalizedVec3 {
        let m = t.backward();
        Vec3::new(
            m[0][0] * self.x() + m[1][0] * self.y() + m[2][0] * self.z(),
            m[0][1] * self.x() + m[1][1] * self.y() + m[2][1] * self.z(),
            m[0][2] * self.x() + m[1][2] * self.y() + m[2][2] * self.z(),
        )
        .normalized()
    }
}

impl From<Vec3> for NormalizedVec3 {
    fn from(vec: Vec3) -> NormalizedVec3 {
        NormalizedVec3::from_vec(vec)
    }
}

