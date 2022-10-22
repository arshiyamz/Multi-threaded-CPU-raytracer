use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::thread::available_parallelism;
use std::thread::Builder;

mod math;
mod utils;

use math::core::*;
use math::vect::Vect;
use math::ray::Ray;
use math::point::Point;
use math::camera::Camera;
use math::sphere::Sphere;
use math::hittable::*;
use math::random::*;
use math::framebuffer::*;
use utils::color::FColor;

const RENDER_HEIGHT: usize = 260;
const RENDER_WIDTH: usize = ((RENDER_HEIGHT as f64) * ASPECT_RATIO) as usize;
const NUM_SAMPLES: usize = 100;

const PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");
const IMAGE_OUT_PATH: &str = "out/output.ppm";

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

fn render_slice(slice: &mut Vec<&mut [FColor]>)
{
    
}


fn main() -> std::io::Result<()>
{
    let mut out_path = PathBuf::from(PROJECT_PATH);
    out_path.push(IMAGE_OUT_PATH);
    let mut file = File::create(out_path)?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", RENDER_WIDTH, RENDER_HEIGHT).as_bytes())?;
    file.write_all(b"255\n")?;

    // Setup Deterministic Random Genrator:
    let mut rand_gen = SimpleDeterministicRandomGenerator::new();

    // Setup Camera:
    let camera = Camera::new();

    let mut fb = FrameBuffer::<RENDER_WIDTH, RENDER_HEIGHT>::new();

    // Setup World:
    let mut world = HittableList::default();
    let little_sphere = Sphere{center: Point::make_new(0.0, 0.0, -1.0), radius: 0.5};
    let big_sphere = Sphere{center: Point::make_new(0.0, -100.5, -1.0), radius: 100.0};
    world.add(&little_sphere);
    world.add(&big_sphere);


    let mut u: f64 = 0.0;
    let mut v: f64 = 0.0;
    let mut ray = Ray::new(&Vect::make_new(0.0, 0.0, 0.0), &Vect::make_new(0.0, 0.0, 0.0));
    let mut color = FColor::make_new(0.0, 0.0, 0.0);
    let mut result = fb.get_slice(0, RENDER_WIDTH.into(), 0, RENDER_HEIGHT.into());

    let thread_builder = Builder::new();
    let num_available_threads = available_parallelism().unwrap().get();
    let x_stride = RENDER_WIDTH / num_available_threads;
    let y_stride = RENDER_HEIGHT / num_available_threads;
    for thread_id in 0..num_available_threads-1
    {
        let mut result = fb.get_slice(thread_id * x_stride, (thread_id + 1) * x_stride, thread_id * y_stride, (thread_id + 1) * y_stride);
        thread_builder.spawn(|| render_slice(&mut result) );
    }

    for height_iterator in (0..RENDER_HEIGHT).rev() // reverse y since top left is -1, 1 in NDC and not -1, -1. 
    {
        for width_iterator in 0..RENDER_WIDTH
        {
            color.reset();
            for sample_number in 0..NUM_SAMPLES
            {
                u = (width_iterator as f64 + rand_gen.rand()) / (RENDER_WIDTH - 1) as f64 - 0.5;
                v = (height_iterator as f64 + rand_gen.rand()) / (RENDER_HEIGHT - 1) as f64 - 0.5;

                ray = camera.get_ray(u, v);

                color += &ray_color(&ray, &world);
            }
            result[height_iterator][width_iterator] = &color / NUM_SAMPLES as f64;
        }
    }
    fb.write_to_file(&mut file)?;

    Ok(())
}
