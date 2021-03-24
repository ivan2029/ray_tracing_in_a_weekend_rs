mod camera;
mod cgmath;
mod color;
mod material;
mod ray;
mod raytrace;
mod scene;
mod shape;

use crate::camera::*;
use crate::cgmath::*;
use crate::color::*;
use crate::material::*;
use crate::raytrace::*;
use crate::scene::*;
use crate::shape::*;

use anyhow::Result;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

use std::{sync::mpsc, thread};

fn main() -> Result<()> {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as _;

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
        sample_count: 500,
        max_depth: 50,
    };

    // scene
    let scene = make_test_scene();

    // log thread
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut rows = vec![0; image_height as usize];
        let mut rows_completed = 0;

        while let Ok(row) = rx.recv() {
            rows[row] += 1;
            if rows[row] == image_width {
                rows_completed += 1;
                println!("finished rows {}", rows_completed);
            }
        }
    });

    // raytrace
    let mut buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);

    buf.par_chunks_exact_mut(3)
        .enumerate()
        .for_each_with(tx, |tx, (pos, pixel)| {
            let x = pos as u32 % image_width;
            let y = pos as u32 / image_width;

            let u = x as f32 / (image_width - 1) as f32;
            let v = 1.0 - (y as f32 / (image_height - 1) as f32);

            let comps = (0..ray_cast_options.sample_count)
                .into_iter()
                .map(|_| {
                    let du = rand::thread_rng().gen_range(-0.5..0.5) / image_width as f32;
                    let dv = rand::thread_rng().gen_range(-0.5..0.5) / image_height as f32;

                    let ray = camera.ray_at(u + du, v + dv);

                    let comps: Vec3 = ray_color(&ray_cast_options, &scene, &ray, 0).into();
                    comps
                })
                .fold(Vec3::ZERO, |a, b| a + b);
            let color: Color = (comps / ray_cast_options.sample_count as f32).into();

            // gamma correct (!?)
            let color = Color::from_rgb(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

            //
            let color = image::Rgb(color.as_u8());
            pixel[0] = color.0[0];
            pixel[1] = color.0[1];
            pixel[2] = color.0[2];

            //
            tx.send(y as usize).unwrap();
        });

    buf.save("target/test.png")?;

    //
    Ok(())
}

//
//
//
fn make_test_scene() -> Scene {
    //
    let mut scene = Scene::new();

    // ground
    {
        let m = scene.insert_material(Lambertian::new(Color::from_rgb(0.5, 0.5, 0.5)));

        let s = scene.insert_shape(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
        });

        scene.insert_object(s, m);
    }

    // predefined spheres
    {
        let m = scene.insert_material(Dielectric::new(1.5));
        let s = scene.insert_shape(Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
        });

        scene.insert_object(s, m);
    }

    {
        let m = scene.insert_material(Lambertian::new(Color::from_rgb(0.4, 0.2, 0.1)));
        let s = scene.insert_shape(Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
        });

        scene.insert_object(s, m);
    }

    {
        let m = scene.insert_material(Metal::new(Color::from_rgb(0.7, 0.6, 0.5), 0.0));
        let s = scene.insert_shape(Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
        });

        scene.insert_object(s, m);
    }

    // random spheres
    {
        let danger = Vec3::new(4.0, 0.2, 0.0);

        for a in -11..11 {
            for b in -11..11 {
                let center = Vec3::new(
                    a as f32 + 0.9 * thread_rng().gen_range(0.0..1.0),
                    0.2,
                    b as f32 + 0.9 * thread_rng().gen_range(0.0..1.0),
                );

                if (center - danger).norm() < 0.9 {
                    continue;
                }

                let m = match thread_rng().gen_range(0..3) {
                    0 => {
                        let albedo = Color::random() * Color::random();
                        scene.insert_material(Lambertian::new(albedo))
                    }
                    1 => {
                        let albedo = Color::from_rgb(0.5, 0.5, 0.5) + 0.5 * Color::random();
                        let fuzz = thread_rng().gen_range(0.0..0.5);
                        scene.insert_material(Metal::new(albedo, fuzz))
                    }
                    2 => scene.insert_material(Dielectric::new(thread_rng().gen_range(1.1..1.9))),
                    _ => unreachable!(),
                };

                let s = scene.insert_shape(Sphere {
                    center,
                    radius: 0.2,
                });

                scene.insert_object(s, m);
            }
        }
    }

    //
    scene
}
