#![allow(clippy::many_single_char_names)]

use crate::cgmath::{angle::*, normal::NormalizedVec3, point3::Point3, transform::Transform, PRECISION};

use rand::{thread_rng, Rng};
use std::ops::{Add, Div, Mul, Neg, Range, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
    ) -> Vec3 {
        assert!(!x.is_nan());
        assert!(!y.is_nan());
        assert!(!z.is_nan());

        Vec3 { x, y, z }
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
    ) -> Vec3 {
        let m = t.forward();
        Vec3::new(
            m[0][0] * self.x + m[0][1] * self.y + m[0][2] * self.z + m[0][3],
            m[1][0] * self.x + m[1][1] * self.y + m[1][2] * self.z + m[1][3],
            m[2][0] * self.x + m[2][1] * self.y + m[2][2] * self.z + m[2][3],
        )
    }

    pub fn dot(
        self,
        other: Vec3,
    ) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(
        self,
        other: Vec3,
    ) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn norm_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    pub fn normalized(self) -> NormalizedVec3 {
        NormalizedVec3::from_vec(self)
    }

    pub fn lerp(
        t: f32,
        u: Vec3,
        v: Vec3,
    ) -> Vec3 {
        (1.0 - t) * u + t * v
    }

    pub fn quadratic(
        t: f32,
        a: Vec3,
        b: Vec3,
        c: Vec3,
    ) -> Vec3 {
        // let ab = lerp(t, a, b);
        // let bc = lerp(t, b, c);
        // lerp(t, &ab, &bc)
        let one_m_t = 1.0 - t;
        (one_m_t * one_m_t) * a + (2.0 * one_m_t * t) * b + (t * t) * c
    }

    pub fn cubic(
        t: f32,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        d: Vec3,
    ) -> Vec3 {
        // let ab = lerp(t, a, b);
        // let bc = lerp(t, b, c);
        // let cd = lerp(t, c, d);
        // let abc = lerp(t, &ab, &bc);
        // let bcd = lerp(t, &bc, &cd);
        // lerp(t, &abc, &bcd)
        let one_m_t = 1.0 - t;
        (one_m_t * one_m_t * one_m_t) * a
            + (3.0 * one_m_t * one_m_t * t) * b
            + (3.0 * one_m_t * t * t) * c
            + (t * t * t) * d
    }

    pub fn is_near_zero(self) -> bool {
        self.x.abs() < PRECISION && self.y.abs() < PRECISION && self.z.abs() < PRECISION
    }

    pub fn angle(
        self,
        other: Vec3,
    ) -> Radians {
        let rads = self.dot(other) / (self.norm() * other.norm());
        Radians(rads)
    }

    pub fn random() -> Vec3 {
        Vec3::new(thread_rng().gen(), thread_rng().gen(), thread_rng().gen())
    }

    pub fn random_range(r: Range<f32>) -> Vec3 {
        Vec3::new(
            thread_rng().gen_range(r.clone()),
            thread_rng().gen_range(r.clone()),
            thread_rng().gen_range(r),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random_range(-1.0..1.0);
            if v.norm_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector() -> NormalizedVec3 {
        Vec3::random_in_unit_sphere().normalized()
    }

    pub fn random_in_hemisphere(normal: NormalizedVec3) -> Vec3 {
        let u = Vec3::random_unit_vector().into();
        if Vec3::dot(u, normal.into()) > 0.0 {
            u
        } else {
            -u
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let v = Vec3::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                0.0,
            );
            if v.norm_squared() < 1.0 {
                return v;
            }
        }
    }
}

// constants
impl Vec3 {
    pub const X: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const Y: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const Z: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

impl From<NormalizedVec3> for Vec3 {
    fn from(value: NormalizedVec3) -> Vec3 {
        value.to_vec3()
    }
}

impl From<Point3> for Vec3 {
    fn from(value: Point3) -> Vec3 {
        value.pos()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(
        self,
        other: Vec3,
    ) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(
        self,
        other: Vec3,
    ) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(
        self,
        c: f32,
    ) -> Vec3 {
        Vec3 {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(
        self,
        v: Vec3,
    ) -> Vec3 {
        v * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(
        self,
        c: f32,
    ) -> Vec3 {
        Vec3 {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
        }
    }
}
