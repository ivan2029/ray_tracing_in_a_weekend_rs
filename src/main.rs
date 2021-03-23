mod cgmath;
mod ray;
mod color;
mod shape;
mod material;
mod scene;

mod playground;

use crate::cgmath::*;
use crate::ray::*;
use crate::color::*;
use crate::shape::*;
use crate::material::*;
use crate::scene::*;

use playground::*;

use anyhow::Result;
use rayon::prelude::*;

fn main() -> Result<()> {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f32 / aspect_ratio) as _;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::ZERO;
    let horizontal = viewport_width * &Vec3::X;
    let vertical = viewport_height * &Vec3::Y;
    let lower_left_corner 
        = &origin 
        - 0.5 * &horizontal 
        - 0.5 * &vertical 
        - focal_length * &Vec3::Z;

    // raytrace options
    let ray_cast_options = RayCastOptions {
        sample_count: 100,
        max_depth: 5,
    };

    // scene
    let scene: Vec<Box<dyn HittableShape>> = vec![
        Box::new(
            Sphere {
                center: -1.0 * &Vec3::Z,
                radius: 0.5
            }
        ),
        Box::new(
            Sphere {
                center: Vec3::new(0.0, -100.5,  -1.0),
                radius: 100.0
            }
        ),
    ];

    let scene_iter = scene.iter()
        .map(|b| b.as_ref());

    // raytrace
    let mut buf = image::ImageBuffer::new(image_width, image_height);

    let mut last_y = std::u32::MAX;
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        if last_y != y {
            last_y = y;
            println!("starting row {} of {}", y + 1, image_height);
        }

        let u = x as f32 / (image_width - 1) as f32;
        let v = 1.0 - (y as f32 / (image_height - 1) as f32);

        let comps = (0 .. ray_cast_options.sample_count).into_par_iter()
            .map(|_| {
                use rand::Rng;
                let du = rand::thread_rng().gen_range(-0.5 .. 0.5) / image_width as f32;
                let dv = rand::thread_rng().gen_range(-0.5 .. 0.5) / image_height as f32;

                let ray = Ray::new(
                    origin.clone(),
                    &lower_left_corner 
                    + (u + du) * &horizontal 
                    + (v + dv) * &vertical - &origin
                );

                let comps: Vec3 = ray_color(&ray_cast_options, scene_iter.clone(), &ray, 0).into();
                comps  
            })
            .reduce(|| Vec3::ZERO, |a, b| &a + &b);
        let color: Color = (&comps / ray_cast_options.sample_count as f32).into();

        //
        *pixel = image::Rgb(color.as_u8());
    } 

    buf.save("target/test.png")?;

    //
    Ok(())
}


fn background_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().y + 1.0); 
    let cs = Vec3::lerp(t, &Vec3::ONE, &Vec3::new(0.5, 0.7, 1.0));
    cs.into()
}

fn nearest_hit<'a, I>(it: I, ray: &Ray, near: f32, far: f32) -> Option<ShapeHit> 
    where I: Iterator<Item=&'a dyn HittableShape>
{
    let inf_hit = ShapeHit {
        t: f32::INFINITY,
        .. Default::default()
    };

    fn choose_nearer(a: ShapeHit, b: ShapeHit) -> ShapeHit {
        if a.t < b.t {
            a
        } else {
            b
        }
    }

    let hit = it.filter_map(|h| h.hit(ray, near, far))
        .fold(inf_hit, choose_nearer);

    if f32::is_infinite(hit.t) {
        None
    } else {
        Some(hit)
    }
}

struct RayCastOptions {
    sample_count: usize,
    max_depth: usize,
}

impl Default for RayCastOptions {
    fn default() -> Self {
        RayCastOptions {
            sample_count: 50,
            max_depth: 8,
        }
    }
}


fn ray_color<'a, I>(options: &RayCastOptions, hittables: I, ray: &Ray, ray_depth: usize) -> Color 
    where I: Iterator<Item=&'a dyn HittableShape> + Clone
{
    if ray_depth > options.max_depth {
        return Color::from_rgb(0.0, 0.0, 0.0);
    }

    let ohit = nearest_hit(hittables.clone(), ray, 0.001, 100.0);

    match ohit {
        Some(hit) => {
            let target = &hit.point + &hit.normal + &Vec3::random_unit_vector();
            let target_ray = Ray::new(hit.point.clone(), &target - &hit.point);
            let target_color = ray_color(options, hittables, &target_ray, ray_depth + 1);
        
            0.5 * target_color  
        },
        None => background_color(ray)
    }
}
