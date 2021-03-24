use crate::cgmath::*;
use crate::color::*;
use crate::ray::*;

use std::fmt::Debug;

//
//
//

fn reflect(u: Vec3, normal: Vec3) -> Vec3 {
    u - (2.0 * Vec3::dot(u, normal)) * normal
}

/*
 * Snell's law :  `eta * theta = eta' * theta'`
 *   where: 
 *     `eta`, `eta'`: refractive index for medium
 *     `theta`, `theta'`: angle between incoming array and surface normal
 *    
 * Example values of `eta`:
 *    air = 1.0
 *    glass = 1.3 to 1.7
 *    diamond = 2.4
 *
 *  `index` is `eta / eta'` 
 */
fn refract(u: Vec3, normal: Vec3, index: f32) -> Vec3 {
    let cos_theta = normal.angle(-u).0.min(1.0);
    let r_out_perp = index * (u + cos_theta * normal);
    let r_out_parallel = (-(1.0 - r_out_perp.norm_squared()).abs().sqrt()) * normal;    
    r_out_perp + r_out_parallel
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
            let candidate = hit.normal + Vec3::random_unit_vector();
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
        let reflected = reflect(*ray_in.direction(), hit.normal);
        let random = self.fuzz * Vec3::random_in_unit_sphere();
        let scatter_direction = reflected + random;

        let ray = Ray::new(hit.point.clone(), scatter_direction);

        let attenuation = self.albedo.clone();

        Some(Scatter { attenuation, ray })
    }
}

//
//
//
#[derive(Debug)]
pub struct Dielectric;

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &ShapeHit) -> Option<Scatter> {
        todo!()
    }
}