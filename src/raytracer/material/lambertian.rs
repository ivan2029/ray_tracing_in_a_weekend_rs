/*
 */
use crate::{
    cgmath::{ray::Ray, vec3::Vec3},
    raytracer::{
        color::Color,
        material::{Material, Scatter},
        shape::ShapeHit,
    },
};

/*
 */
#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit: &ShapeHit,
    ) -> Option<Scatter> {
        let scatter_direction = {
            let candidate = hit.normal.to_vec3() + Vec3::random_unit_vector().to_vec3();
            if candidate.is_near_zero() {
                hit.normal.to_vec3()
            } else {
                candidate
            }
        };

        let ray = Ray::new(hit.point, scatter_direction);

        let attenuation = self.albedo;

        Some(Scatter { attenuation, ray })
    }
}
