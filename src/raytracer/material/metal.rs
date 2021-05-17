/*
 */
use crate::{
    cgmath::{ray::Ray, vec3::Vec3},
    raytracer::{
        color::Color,
        material::{helpers::reflect, Material, Scatter},
        shape::ShapeHit,
    },
};

/*
 */
#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f32, // in (0.0, 1.0)
}

impl Metal {
    pub fn new(
        albedo: Color,
        fuzz: f32,
    ) -> Metal {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &ShapeHit,
    ) -> Option<Scatter> {
        let reflected = reflect(ray_in.direction(), hit.normal);
        let random = self.fuzz * Vec3::random_in_unit_sphere();
        let scatter_direction = reflected + random;

        let ray = Ray::new(hit.point, scatter_direction);

        let attenuation = self.albedo;

        Some(Scatter { attenuation, ray })
    }
}
