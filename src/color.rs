use std::ops::{Add, Mul, Sub};

use crate::cgmath::Vec3;

#[derive(Debug, Clone, Copy)]
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

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::from_rgb(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color::from_rgb(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, c: f32) -> Color {
        Color::from_rgb(self.r * c, self.g * c, self.b * c)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, c: Color) -> Color {
        c * self
    }
}
