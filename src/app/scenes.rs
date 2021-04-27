use std::collections::HashMap;

use crate::raytracer::scene::Scene;

//
//
//
pub type SceneCreator = fn() -> Scene;
pub type SceneCreators = HashMap<&'static str, SceneCreator>;

pub fn scene_creators() -> SceneCreators {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Book 1 final scene",
        make_book_1_final_scene as SceneCreator,
    );

    hash_map
}

//
//
//
fn make_book_1_final_scene() -> Scene {
    use crate::cgmath::*;
    use crate::raytracer::color::*;
    use crate::raytracer::material::*;
    use crate::raytracer::scene::*;
    use crate::raytracer::shape::*;

    use rand::{thread_rng, Rng};

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
