use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use rand::{thread_rng, Rng};
use std::{
    f32::consts::{FRAC_1_PI, PI},
    ops::Range,
};

//
//
//
macro_rules! impl_norms {
    ($vt:tt) => {
        ///
        pub fn norm_squared(&self) -> f32 {
            self.dot(self)
        }

        ///
        pub fn norm(&self) -> f32 {
            self.norm_squared().sqrt()
        }

        ///
        pub fn normalized(&self) -> Vec3 {
            self / self.norm()
        }
    };
}

macro_rules! impl_interpolation {
    ($vt:tt) => {
        ///
        pub fn lerp(t: f32, u: &$vt, v: &$vt) -> $vt {
            (1.0 - t) * u + t * v
        }

        ///
        pub fn quadratic(t: f32, a: &$vt, b: &$vt, c: &$vt) -> $vt {
            // let ab = lerp(t, a, b);
            // let bc = lerp(t, b, c);
            // lerp(t, &ab, &bc)
            let one_m_t = 1.0 - t;
            (one_m_t * one_m_t) * a + (2.0 * one_m_t * t) * b + (t * t) * c
        }

        ///
        pub fn cubic(t: f32, a: &$vt, b: &$vt, c: &$vt, d: &$vt) -> $vt {
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
    };
}

//
//
//
const FRAC_1_180: f32 = 1.0 / 180.0;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Radians(pub f32);

impl Into<Degrees> for Radians {
    fn into(self) -> Degrees {
        Degrees(FRAC_1_PI * 180.0 * self.0)
    }
}

impl_op_ex!(+ |a: Radians, b: Radians| -> Radians { Radians(a.0 + b.0) });
impl_op_ex!(-|a: Radians, b: Radians| -> Radians { Radians(a.0 - b.0) });

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Degrees(pub f32);

impl Into<Radians> for Degrees {
    fn into(self) -> Radians {
        Radians(PI * FRAC_1_180 * self.0)
    }
}

impl_op_ex!(+ |a: Degrees, b: Degrees| -> Degrees { Degrees(a.0 + b.0) });
impl_op_ex!(-|a: Degrees, b: Degrees| -> Degrees { Degrees(a.0 - b.0) });

//
//
//

///
#[derive(Debug, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    ///
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    ///
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    ///
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    impl_norms!(Self);
    impl_interpolation!(Self);

    ///
    pub fn near_zero(&self) -> bool {
        let precision = 1e-8;
        self.x.abs() < precision && self.y.abs() < precision && self.z.abs() < precision
    }

    ///
    pub fn random() -> Vec3 {
        Vec3::new(thread_rng().gen(), thread_rng().gen(), thread_rng().gen())
    }

    ///
    pub fn random_range(r: Range<f32>) -> Vec3 {
        Vec3::new(
            thread_rng().gen_range(r.clone()),
            thread_rng().gen_range(r.clone()),
            thread_rng().gen_range(r),
        )
    }

    ///
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut v = Vec3::random_range(-1.0..1.0);
        while v.norm_squared() > 1.0 {
            v = Vec3::random_range(-1.0..1.0);
        }
        v
    }

    ///
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalized()
    }

    ///
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let u = Vec3::random_unit_vector();
        if Vec3::dot(&u, normal) > 0.0 {
            u
        } else {
            -u
        }
    }
}

// constants
impl Vec3 {
    ///
    pub const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);

    ///
    pub const Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    ///
    pub const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    ///
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    ///
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
}

impl_op_ex!(-|u: &Vec3| -> Vec3 {
    Vec3 {
        x: -u.x,
        y: -u.y,
        z: -u.z,
    }
});

impl_op_ex!(+ |u: &Vec3, v: &Vec3| -> Vec3 {
    Vec3 {
        x: u.x + v.x,
        y: u.y + v.y,
        z: u.z + v.z,
    }
});

impl_op_ex!(-|u: &Vec3, v: &Vec3| -> Vec3 {
    Vec3 {
        x: u.x - v.x,
        y: u.y - v.y,
        z: u.z - v.z,
    }
});

impl_op_ex_commutative!(*|u: &Vec3, c: f32| -> Vec3 {
    Vec3 {
        x: u.x * c,
        y: u.y * c,
        z: u.z * c,
    }
});

impl_op_ex!(/ |u: &Vec3, c: f32| -> Vec3 {
    Vec3 {
        x: u.x / c,
        y: u.y / c,
        z: u.z / c,
    }
});
