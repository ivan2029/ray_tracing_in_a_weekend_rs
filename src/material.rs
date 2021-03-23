use crate::cgmath::*;
use crate::color::*;
use crate::ray::*;

use std::fmt::Debug;

//
//
//

fn reflect(u: &Vec3, normal: &Vec3) -> Vec3 {
    u - (2.0 * Vec3::dot(u, normal)) * normal
}

//
//
//
#[derive(Debug, Clone)]
pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray_in: &Ray, hit: &ShapeHit) -> Option<Scatter>;
}

//
//
//
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &ShapeHit) -> Option<Scatter> {
        let scatter_direction = {
            let candidate = &hit.normal + &Vec3::random_unit_vector();
            if candidate.near_zero() {
                hit.normal.clone()
            } else {
                candidate
            }
        };

        let ray = Ray::new(hit.point.clone(), scatter_direction);

        let attenuation = self.albedo.clone();

        Some(Scatter { attenuation, ray })
    }
}

//
//
//
#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32, // in (0.0, 1.0)
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &ShapeHit) -> Option<Scatter> {
        let reflected = reflect(&ray_in.direction(), &hit.normal);
        let random = self.fuzz * &Vec3::random_in_unit_sphere();
        let scatter_direction = &reflected + &random;

        let ray = Ray::new(hit.point.clone(), scatter_direction);

        let attenuation = self.albedo.clone();

        Some(Scatter { attenuation, ray })
    }
}

//
//
//
