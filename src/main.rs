use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const RENDER_WIDTH: i32 = 1280;
const RENDER_HEIGHT: i32 = 720;
const ASPECT_RATIO: f64 = RENDER_WIDTH as f64 / RENDER_HEIGHT as f64;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;

const FOCAL_LENGTH: f64 = 1.0;

const PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");
const IMAGE_OUT_PATH: &str = "out/output.ppm";

mod math;
mod utils;

use math::core::*;
use math::vect::Vect;
use math::ray::Ray;
use math::point::Point;
use math::camera::Camera;
use math::sphere::Sphere;
use math::hittable::*;
use utils::color::FColor;

// Background Gradient:

fn ray_color(ray: &Ray, world: &dyn Hittable) -> FColor
{
    let (hit, hit_result) = world.hit(ray, 0.0, INFINITY);
    if hit
    {
        return 0.5 * &(&hit_result.normal + &FColor::new_color(1.0, 1.0, 1.0));
    }
    let dir = ray.direction().get_normalized();
    let y = 0.5 * dir.y() + 0.5;
    &FColor::new_color(0.44, 0.71, 0.81) + &(y * &FColor::new_color(0.44, 0.26, 0.19))
}


fn main() -> std::io::Result<()>
{
    let mut out_path = PathBuf::from(PROJECT_PATH);
    out_path.push(IMAGE_OUT_PATH);
    let mut file = File::create(out_path)?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", RENDER_WIDTH, RENDER_HEIGHT).as_bytes())?;
    file.write_all(b"255\n")?;


    // Setup Camera:
    let camera = Camera::new();

    // Setup World:
    let mut world = HittableList::default();
    let little_sphere = Sphere{center: Point::new_point(0.0, 0.0, -1.0), radius: 0.5};
    let big_sphere = Sphere{center: Point::new_point(0.0, -100.5, -1.0), radius: 100.0};
    world.add(&little_sphere);
    world.add(&big_sphere);

    for height_iterator in (0..RENDER_HEIGHT).rev() // reverse y since top left is -1, 1 in NDC and not -1, -1. 
    {
        for width_iterator in 0..RENDER_WIDTH
        {
            let x = width_iterator as f64 / (RENDER_WIDTH - 1) as f64 - 0.5;
            let y = height_iterator as f64 / (RENDER_HEIGHT - 1) as f64 - 0.5;

            let x = x * VIEWPORT_WIDTH;
            let y = y * VIEWPORT_HEIGHT;

            let ray = Ray::new(&camera.origin, &(&camera.origin + &(&(x * &camera.right) + &(&(y * &camera.up) + &Vect::new_vect(0.0, 0.0, -FOCAL_LENGTH)))));
            
            let color = ray_color(&ray, &world);
            file.write_all(color.display_color().as_bytes())?;
        }
    }

    Ok(())
}
