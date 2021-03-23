use crate::cgmath::Vec3;

use auto_ops::{impl_op_ex, impl_op_ex_commutative};


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