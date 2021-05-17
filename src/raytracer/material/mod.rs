/*
 */
pub mod dielectric;
pub mod lambertian;
pub mod metal;

mod helpers;

/*
 */
use crate::{
    cgmath::ray::Ray,
    raytracer::{color::Color, shape::ShapeHit},
};
use std::fmt::Debug;

/*
*/
#[derive(Debug, Clone)]
pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync + Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &ShapeHit,
    ) -> Option<Scatter>;
}

/*
 */
crate::define_variant!( MaterialDef: Clone {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric)
});

impl Material for MaterialDef {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &ShapeHit,
    ) -> Option<Scatter> {
        match self {
            MaterialDef::Lambertian(inner) => inner.scatter(ray_in, hit),
            MaterialDef::Metal(inner) => inner.scatter(ray_in, hit),
            MaterialDef::Dielectric(inner) => inner.scatter(ray_in, hit),
        }
    }
}
