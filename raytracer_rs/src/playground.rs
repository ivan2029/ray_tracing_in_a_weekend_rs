use auto_ops::{impl_op_ex, impl_op_ex_commutative};

///
#[derive(Debug, Clone)]
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

    ///
    pub fn lerp(&self, v: &Vec3, t: f32) -> Vec3 {
        (1.0 - t) * self + t * v
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
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn as_u8(&self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
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
            r: components.x,
            g: components.y,
            b: components.z,
        }
    }
}

//
//
//

#[derive(Debug, Clone)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}


pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
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
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let center_origin = ray.origin() - &self.center;

        let a = ray.direction().norm_squared();
        let b = 2.0 * Vec3::dot(&center_origin, ray.direction());
        let c = center_origin.norm_squared() - self.radius * self.radius;
    
        let d = b * b - 4.0 * a * c;
    
        if d < 0.0 {
            return None;
        } 
    
        let d = d.sqrt();

        let t1 = (- b - d) / (2.0 * a);
        let t2 = (- b + d) / (2.0 * a);
        let t = match (t1 > 0.0, t2 > 0.0) {
            (true, true) => f32::min(t1, t2),
            (true, false) => t1,
            (false, true) => t2,
            _ => return None
        };

        let point = ray.at(t);
        let normal = (&point - &self.center).normalized();

        Some(Hit {
            point, 
            normal,
            t,
        })
    }
}