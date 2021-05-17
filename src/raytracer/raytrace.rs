use crate::{
    cgmath::{ray::Ray, vec3::Vec3},
    raytracer::{camera::Camera, color::Color, scene::Scene},
};
use std::time::Duration;

/*
 */
#[derive(Debug)]
pub struct Chunk {
    pub x: usize,
    pub y: usize,
    pub duration: Duration,
    pub pixels: Vec<Color>,
}

#[derive(Debug)]
pub struct TraceOptions {
    image_width: usize,
    image_height: usize,
    sample_count: usize,
    max_depth: usize,
    chunk_size: usize,
}

/*
 */
fn background_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().y() + 1.0);
    let cs = Vec3::lerp(t, Vec3::ONE, Vec3::new(0.5, 0.7, 1.0));
    cs.into()
}

/*
 */
pub fn trace_ray(
    trace_options: &TraceOptions,
    scene: &Scene,
    ray: &Ray,
    ray_depth: usize,
) -> Color {
    // if ray_depth > trace_options.max_depth {
    //     return Color::from_rgb(0.0, 0.0, 0.0);
    // }

    // let ohit = scene.nearest_hit(ray, 0.001, 100.0);

    // match ohit {
    //     Some(hit) => {
    //         let mat = scene.get_material(hit.object);

    //         if let Some(scatter) = mat.scatter(ray, &hit.shape_hit) {
    //             let in_color = trace_ray(trace_options, scene, &scatter.ray, ray_depth + 1);
    //             Color::from_rgb(
    //                 scatter.attenuation.r() * in_color.r(),
    //                 scatter.attenuation.g() * in_color.g(),
    //                 scatter.attenuation.b() * in_color.b(),
    //             )
    //         } else {
    //             Color::from_rgb(0.0, 0.0, 0.0)
    //         }
    //     }
    //     None => background_color(ray),
    // }
    todo!()
}

/*
 */
pub fn raytrace_task<F>(
    trace_options: &TraceOptions,
    scene: &Scene,
    camera: &dyn Camera,
    on_chunk: F,
) where
    F: Fn(Option<Chunk>) + Send + Sync,
{
    on_chunk(None);

    // //
    // use crate::{cgmath::*, raytracer::camera::Camera};

    // use rand::{thread_rng, Rng};

    // use rayon::prelude::*;

    // use std::time::Instant;

    // //
    // let aspect_ratio = image_width as f32 / image_height as f32;

    // let camera = {
    //     let eye = Vec3::new(13.0, 2.0, 3.0);
    //     let target = Vec3::ZERO;
    //     let up = Vec3::Y;
    //     let vertical_fov = Degrees(20.0).into();
    //     let aperture = 0.1;
    //     let focal_distance = 10.0;

    //     &Camera::new(
    //         eye,
    //         target,
    //         up,
    //         vertical_fov,
    //         aspect_ratio,
    //         aperture,
    //         focal_distance,
    //     )
    // };

    // //
    // let chunks = {
    //     let x_chunks = (image_width + chunk_size - 1) / chunk_size;
    //     let y_chunks = (image_height + chunk_size - 1) / chunk_size;

    //     let mut chunks = Vec::with_capacity(x_chunks * y_chunks);

    //     for y in 0..y_chunks {
    //         for x in 0..x_chunks {
    //             chunks.push((x, y));
    //         }
    //     }

    //     chunks
    // };

    // chunks.into_par_iter().for_each(|(x, y)| {
    //     let mut colors = vec![Color::from_rgb(0.0, 0.0, 0.0); chunk_size * chunk_size];

    //     let begin = Instant::now();

    //     for j in 0..chunk_size {
    //         for i in 0..chunk_size {
    //             let tx = x * chunk_size + i;
    //             let ty = y * chunk_size + j;

    //             if tx >= image_width || ty >= image_height {
    //                 continue;
    //             }

    //             let u = tx as f32 / (image_width - 1) as f32;
    //             let v = 1.0 - (ty as f32 / (image_height - 1) as f32);

    //             let mut comps = Vec3::ZERO;
    //             for _ in 0..sample_count {
    //                 let du = thread_rng().gen_range(-0.5..0.5) / image_width as f32;
    //                 let dv = thread_rng().gen_range(-0.5..0.5) / image_height as f32;

    //                 let ray = camera.ray_at(u + du, v + dv);

    //                 comps = comps + ray_color(scene, &ray, 0, sample_count, max_depth).into();
    //             }

    //             comps = comps / sample_count as f32;

    //             // color correction ?
    //             comps.x = comps.x.sqrt();
    //             comps.y = comps.y.sqrt();
    //             comps.z = comps.z.sqrt();

    //             colors[j * chunk_size + i] = comps.into();
    //         }
    //     }

    //     let end = Instant::now();

    //     let _ = sender.send(Chunk {
    //         x,
    //         y,
    //         duration: end - begin,
    //         pixels: colors,
    //     });
    // });
}
