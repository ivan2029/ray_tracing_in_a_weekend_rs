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

    // scene
    let sphere = Sphere {
        center: -1.0 * &Vec3::Z,
        radius: 0.5
    };

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

        let ray = Ray::new(
            origin.clone(),
            &lower_left_corner + u * &horizontal + v * &vertical - &origin
        );

        //
        let color = if let Some(hit) = sphere.hit(&ray) {
            (0.5 * (&hit.normal + &Vec3::ONE)).into()
        } else {
            background_color(&ray)
        };

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

