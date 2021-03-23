use crate::ray::*;
use crate::color::*;
//
//
//
#[derive(Debug, Clone)]
pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &ShapeHit, attenuation: &Color) -> Option<Scatter>;    
}

//
//
//
// struct DiffuseV1;

