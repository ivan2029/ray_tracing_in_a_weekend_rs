/*
 */
use crate::cgmath::{normal::NormalizedVec3, vec3::Vec3};

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
pub fn refract(
    u: Vec3,
    normal: NormalizedVec3,
    refraction_ratio: f32,
) -> Vec3 {
    let cos_theta = Vec3::angle(-u, normal.to_vec3()).0.min(1.0);
    let r_out_perp = refraction_ratio * (u + cos_theta * normal.to_vec3());
    let r_out_parallel = {
        let r = 1.0 - r_out_perp.norm_squared();
        let r = -r.abs().sqrt();
        r * normal.to_vec3()
    };
    r_out_perp + r_out_parallel
}

/*
 * Schlick approximation
 */
pub fn reflectance(
    cosine: f32,
    refractive_index: f32,
) -> f32 {
    let r = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r = r * r;
    r + (1.0 - r) * (1.0 - cosine).powi(5)
}

/*
 */
pub fn reflect(
    u: Vec3,
    normal: NormalizedVec3,
) -> Vec3 {
    u - (2.0 * Vec3::dot(u, normal.to_vec3())) * normal.to_vec3()
}
