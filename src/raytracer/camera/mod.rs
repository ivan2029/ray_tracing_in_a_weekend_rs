/*
 */
mod perspective;

/*
 */
use crate::cgmath::ray::Ray;

/*
 * `s` - target image `x` value
 * `t` - target image `y` value
 */
pub trait Camera {
    fn ray_at(
        &self,
        s: f32,
        t: f32,
    ) -> Ray;
}

/*
 */
crate::define_variant!( CameraDef {
    PerspectiveCamera(perspective::PerspectiveCamera)
});
