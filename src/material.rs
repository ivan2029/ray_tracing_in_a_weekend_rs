use rand::Rng;

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
fn refract(u: Vec3, normal: Vec3, refraction_ratio: f32) -> Vec3 {
    let cos_theta = Vec3::angle(-u, normal).0.min(1.0);
    let r_out_perp = refraction_ratio * (u + cos_theta * normal);
    let r_out_parallel = {
        let r = 1.0 - r_out_perp.norm_squared();
        let r = -r.abs().sqrt();
        let r = r * normal;
        r
    };
    r_out_perp + r_out_parallel
}

/*
 * Schlick approximation
 */
fn reflectance(cosine: f32, refractive_index: f32) -> f32 {
    let r = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r = r * r;
    let r = r + (1.0 - r) * (1.0 - cosine).powi(5);
    r
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
                hit.normal
            } else {
                candidate
            }
        };

        let ray = Ray::new(hit.point, scatter_direction);

        let attenuation = self.albedo;

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

        let ray = Ray::new(hit.point, scatter_direction);

        let attenuation = self.albedo;

        Some(Scatter { attenuation, ray })
    }
}

//
//
//
#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &ShapeHit) -> Option<Scatter> {
        let attenuation = Color::from_rgb(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = Vec3::dot(-*ray_in.direction(), hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance_too_high =
            reflectance(cos_theta, self.refraction_index) > rand::thread_rng().gen_range(0.0..1.0);

        let direction = if cannot_refract || reflectance_too_high {
            reflect(*ray_in.direction(), hit.normal)
        } else {
            refract(*ray_in.direction(), hit.normal, refraction_ratio)
        };

        let ray = Ray::new(hit.point, direction);

        Some(Scatter { attenuation, ray })
    }
}
