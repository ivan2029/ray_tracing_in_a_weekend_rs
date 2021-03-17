mod playground;

use playground::*;

use anyhow::Result;

fn main() -> Result<()> {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
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
    let samples_per_pixel = 5;


    // scene
    
    let scene: Vec<Box<dyn Hittable>> = vec![
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

        //
        let mut colors_comps = Vec3::ZERO;

        for _ in 0..samples_per_pixel {
            use rand::Rng;
            let du = rand::thread_rng().gen_range(-0.5 .. 0.5) / image_width as f32;
            let dv = rand::thread_rng().gen_range(-0.5 .. 0.5) / image_height as f32;

            let ray = Ray::new(
                origin.clone(),
                &lower_left_corner 
                + (u + du) * &horizontal 
                + (v + dv) * &vertical - &origin
            );

            let color = if let Some(hit) = nearest_hit(scene_iter.clone(), &ray, 0.1, 1000.0) {
                (0.5 * (&hit.normal + &Vec3::ONE)).into()
            } else {
                background_color(&ray)
            };

            colors_comps = &colors_comps + &color.into();
        }
        
        colors_comps = colors_comps / samples_per_pixel as f32;
        
        let color: Color = colors_comps.into();

        //
        *pixel = image::Rgb(color.as_u8());
    } 

    buf.save("target/test.png")?;

    //
    Ok(())
}


fn background_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().y + 1.0); 
    let cs = Vec3::lerp(&Vec3::ONE, &Vec3::new(0.5, 0.7, 1.0), t);
    cs.into()
}

fn nearest_hit<'a, I>(it: I, ray: &Ray, near: f32, far: f32) -> Option<Hit> 
    where I: Iterator<Item=&'a dyn Hittable>
{
    let inf_hit = Hit {
        t: f32::INFINITY,
        .. Default::default()
    };

    fn choose_nearer(a: Hit, b: Hit) -> Hit {
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