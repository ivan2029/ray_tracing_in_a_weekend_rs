mod cgmath;
mod raytracer;

use crate::cgmath::*;
use crate::raytracer::{camera::*, color::*, make_book_1_final_scene, raytrace::*};

use anyhow::Result;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
use rayon::{prelude::*, ThreadPool, ThreadPoolBuilder};

use std::sync::mpsc;

fn main() -> Result<()> {
    //
    init_logger();

    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as _;

    let (receiver, _thread_pool) = start_worker(image_width, image_height, aspect_ratio)?;

    //
    let mut buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);

    let mut rows = vec![0; image_height as usize];
    let mut rows_completed = 0;
    while let Ok((x, y, color)) = receiver.recv() {
        //
        rows[y as usize] += 1;
        if rows[y as usize] == image_width {
            rows_completed += 1;
            log::info!("finished rows {}", rows_completed);
        }

        //
        *buf.get_pixel_mut(x, y) = Rgb(color.as_u8());
    }

    buf.save("target/test.png")?;

    //
    Ok(())
}

fn init_logger() {
    use env_logger::*;

    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).init();
}

fn start_worker(
    image_width: u32,
    image_height: u32,
    aspect_ratio: f32,
) -> Result<(mpsc::Receiver<(u32, u32, Color)>, ThreadPool)> {
    //
    let thread_pool = ThreadPoolBuilder::new().build()?;

    let (sender, receiver) = mpsc::channel();

    //
    thread_pool.spawn(move || {
        // camera
        let camera = {
            let eye = Vec3::new(13.0, 2.0, 3.0);
            let target = Vec3::ZERO;
            let up = Vec3::Y;
            let vertical_fov = Degrees(20.0).into();
            let aperture = 0.1;
            let focal_distance = 10.0;

            Camera::new(
                eye,
                target,
                up,
                vertical_fov,
                aspect_ratio,
                aperture,
                focal_distance,
            )
        };

        // raytrace options
        let ray_cast_options = RayCastOptions {
            sample_count: 50,
            max_depth: 5,
        };

        // scene
        let scene = make_book_1_final_scene();

        // raytrace
        (0..image_width * image_height)
            .into_par_iter()
            .for_each_with(sender, |sender, pos| {
                let x = pos as u32 % image_width;
                let y = pos as u32 / image_width;

                let u = x as f32 / (image_width - 1) as f32;
                let v = 1.0 - (y as f32 / (image_height - 1) as f32);

                let comps = (0..ray_cast_options.sample_count)
                    .into_iter()
                    .map(|_| {
                        let du = thread_rng().gen_range(-0.5..0.5) / image_width as f32;
                        let dv = thread_rng().gen_range(-0.5..0.5) / image_height as f32;

                        let ray = camera.ray_at(u + du, v + dv);

                        let comps: Vec3 = ray_color(&ray_cast_options, &scene, &ray, 0).into();
                        comps
                    })
                    .fold(Vec3::ZERO, |a, b| a + b);
                let color: Color = (comps / ray_cast_options.sample_count as f32).into();

                // gamma correct (!?)
                let color = Color::from_rgb(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

                //
                sender.send((x, y, color)).unwrap();
            });
    });

    //
    Ok((receiver, thread_pool))
}
