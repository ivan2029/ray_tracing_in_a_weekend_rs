mod cgmath;
mod color;
mod material;
mod ray;
mod raytrace;
mod scene;
mod shape;

use crate::cgmath::*;
use crate::color::*;
use crate::material::*;
use crate::ray::*;
use crate::raytrace::*;
use crate::scene::*;
use crate::shape::*;

use anyhow::Result;
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

use std::{sync::mpsc, thread};

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
    let lower_left_corner = &origin - 0.5 * &horizontal - 0.5 * &vertical - focal_length * &Vec3::Z;

    // raytrace options
    let ray_cast_options = RayCastOptions {
        sample_count: 100,
        max_depth: 100,
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
                    use rand::Rng;
                    let du = rand::thread_rng().gen_range(-0.5..0.5) / image_width as f32;
                    let dv = rand::thread_rng().gen_range(-0.5..0.5) / image_height as f32;

                    let ray = Ray::new(
                        origin.clone(),
                        &lower_left_corner + (u + du) * &horizontal + (v + dv) * &vertical
                            - &origin,
                    );

                    let comps: Vec3 = ray_color(&ray_cast_options, &scene, &ray, 0).into();
                    comps
                })
                .fold(Vec3::ZERO, |a, b| &a + &b);
            let color: Color = (&comps / ray_cast_options.sample_count as f32).into();

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
    use crate::cgmath::*;

    let mut scene = Scene::new();

    let grey_diffuse = scene.insert_material(Lambertian::new(Color::from_rgb(0.5, 0.5, 0.5)));

    let green_metal = scene.insert_material(Metal::new(Color::from_rgb(0.0, 1.0, 0.0), 0.8));
    let red_metal = scene.insert_material(Metal::new(Color::from_rgb(1.0, 0.0, 0.0), 0.2));

    let ground = scene.insert_shape(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    let s1 = scene.insert_shape(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });

    let s2 = scene.insert_shape(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
    });

    let s3 = scene.insert_shape(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
    });

    scene.insert_object(ground, grey_diffuse);
    scene.insert_object(s1, grey_diffuse);
    scene.insert_object(s2, green_metal);
    scene.insert_object(s3, red_metal);

    scene
}
