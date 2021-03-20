use std::{
    f32::consts::{PI, FRAC_1_PI},
    ops::Range,
};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use rand::{Rng, thread_rng};

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
    }
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
    }
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
        Degrees( FRAC_1_PI * 180.0 * self.0)
    }
}

impl_op_ex!(+ |a: Radians, b: Radians| -> Radians { Radians(a.0 + b.0) });
impl_op_ex!(- |a: Radians, b: Radians| -> Radians { Radians(a.0 - b.0) });

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Degrees(pub f32);

impl Into<Radians> for Degrees {
    fn into(self) -> Radians {
        Radians(PI * FRAC_1_180 * self.0)
    }
}

impl_op_ex!(+ |a: Degrees, b: Degrees| -> Degrees { Degrees(a.0 + b.0) });
impl_op_ex!(- |a: Degrees, b: Degrees| -> Degrees { Degrees(a.0 - b.0) });

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
        Vec3 {x, y, z}
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
    pub fn random() -> Vec3 {
        Vec3::new(
            thread_rng().gen(), 
            thread_rng().gen(), 
            thread_rng().gen(),
        )
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
        let mut v = Vec3::random_range(-1.0 .. 1.0);
        while v.norm_squared() > 1.0 {
            v = Vec3::random_range(-1.0 .. 1.0);
        }
        v
    }

    ///
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalized()
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

impl_op_ex!(+ |u: &Vec3, v: &Vec3| -> Vec3 { 
    Vec3 { 
        x: u.x + v.x,
        y: u.y + v.y,
        z: u.z + v.z,
    }
});

impl_op_ex!(- |u: &Vec3, v: &Vec3| -> Vec3 { 
    Vec3 { 
        x: u.x - v.x,
        y: u.y - v.y,
        z: u.z - v.z,
    }
});

impl_op_ex_commutative!(* |u: &Vec3, c: f32| -> Vec3 {
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
        &self.origin + t * &self.direction
    }

}

//
//
//
#[derive(Debug, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { 
            r: r.clamp(0.0, 1.0), 
            g: g.clamp(0.0, 1.0), 
            b: b.clamp(0.0, 1.0), 
        }
    }

    pub fn as_u8(&self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }
}

impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}

impl From<Vec3> for Color {
    fn from(components: Vec3) -> Color {
        Color { 
            r: components.x.clamp(0.0, 1.0),
            g: components.y.clamp(0.0, 1.0),
            b: components.z.clamp(0.0, 1.0),
        }
    }
}

impl_op_ex!(+ |a: &Color, b: &Color| -> Color {
    Color::from_rgb(
        a.r + b.r,
        a.g + b.g,
        a.b + b.b,
    )
});

impl_op_ex!(- |a: &Color, b: &Color| -> Color {
    Color::from_rgb(
        a.r + b.r,
        a.g + b.g,
        a.b + b.b,
    )
});

impl_op_ex_commutative!(* |c: f32, a: &Color| -> Color {
    Color::from_rgb(c * a.r, c * a.g, c * a.b)
});

//
//
//

#[derive(Debug, Clone, Default)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    // pub material_id: usize,
}


pub trait Hittable : Sync + Send {
    fn hit(&self, ray: &Ray, near: f32, far: f32) -> Option<Hit>;
}

//
//
//
#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, near: f32, far: f32) -> Option<Hit> {
        let center_origin = ray.origin() - &self.center;

        let a = ray.direction().norm_squared();
        let half_b = Vec3::dot(&center_origin, ray.direction());
        let c = center_origin.norm_squared() - self.radius * self.radius;
    
        let d = half_b * half_b -  a * c;
    
        if d < 0.0 {
            return None;
        } 
    
        let d = d.sqrt();

        let t1 = (- half_b - d) / a;
        let t2 = (- half_b + d) / a;
        let t = match (t1 > 0.0, t2 > 0.0) {
            (true, true) => f32::min(t1, t2),
            (true, false) => t1,
            (false, true) => t2,
            _ => return None
        };

        if !(near < t && t < far) {
            return None;
        } 

        let point = ray.at(t);
        let normal = (&point - &self.center).normalized();

        Some(Hit {
            point, 
            normal,
            t,
        })
    }
}

//
//
//
// #[derive(Debug, Clone)]
// pub struct Scatter {
//     pub attenuation: Color,
//     pub ray: Ray,
// }

// pub trait Material {
//     fn scatter(&self, ray_in: &Ray, hit: &Hit, attenuation: &Color) -> Option<Scatter>;    
// }

// //
// //
// //
// pub struct World {
//     shapes: Vec<Box<dyn Hittable>>,
//     materials: Vec<Box<dyn Material>>,
// }



