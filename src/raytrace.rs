use crate::cgmath::*;
use crate::color::*;
use crate::ray::*;
use crate::scene::*;

pub struct RayCastOptions {
    pub sample_count: usize,
    pub max_depth: usize,
}

impl Default for RayCastOptions {
    fn default() -> Self {
        RayCastOptions {
            sample_count: 50,
            max_depth: 8,
        }
    }
}

pub fn ray_color(options: &RayCastOptions, scene: &Scene, ray: &Ray, ray_depth: usize) -> Color {
    if ray_depth > options.max_depth {
        return Color::from_rgb(0.0, 0.0, 0.0);
    }

    let ohit = scene.nearest_hit(ray, 0.001, 100.0);

    match ohit {
        Some(hit) => {
            let mat = scene.get_material(hit.object);

            let color = if let Some(scatter) = mat.scatter(ray, &hit.shape_hit) {
                let in_color = ray_color(options, scene, &scatter.ray, ray_depth + 1);
                Color::from_rgb(
                    scatter.attenuation.r() * in_color.r(),
                    scatter.attenuation.g() * in_color.g(),
                    scatter.attenuation.b() * in_color.b(),
                )
            } else {
                Color::from_rgb(0.0, 0.0, 0.0)
            };

            color
        }
        None => background_color(ray),
    }
}

fn background_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().y + 1.0);
    let cs = Vec3::lerp(t, &Vec3::ONE, &Vec3::new(0.5, 0.7, 1.0));
    cs.into()
}
