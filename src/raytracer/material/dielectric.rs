/*
 */
use crate::{
    cgmath::{ray::Ray, vec3::Vec3},
    raytracer::{
        color::Color,
        material::{
            helpers::{reflect, reflectance, refract},
            Material, Scatter,
        },
        shape::ShapeHit,
    },
};
use rand::Rng;

/*
 */
#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &ShapeHit,
    ) -> Option<Scatter> {
        let attenuation = Color::from_rgb(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = Vec3::dot(-ray_in.direction(), hit.normal.to_vec3()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance_too_high =
            reflectance(cos_theta, self.refraction_index) > rand::thread_rng().gen_range(0.0..1.0);

        let direction = if cannot_refract || reflectance_too_high {
            reflect(ray_in.direction(), hit.normal)
        } else {
            refract(ray_in.direction(), hit.normal, refraction_ratio)
        };

        let ray = Ray::new(hit.point, direction);

        Some(Scatter { attenuation, ray })
    }
}
