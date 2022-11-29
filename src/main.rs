#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::thread::available_parallelism;
use std::thread::Builder;
use std::sync::Arc;

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
use math::material::*;
use utils::color::FColor;

const RENDER_HEIGHT: usize = 720;
const RENDER_WIDTH: usize = ((RENDER_HEIGHT as f64) * ASPECT_RATIO) as usize;
const NUM_SAMPLES: usize = 500;
const MAX_CHILD_RAYS: u32 = 50;

const PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");
const IMAGE_OUT_PATH: &str = "out/output.ppm";

// Background Gradient:

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> FColor
{
    if depth <= 0
    {
        return FColor::make_new(0.0, 0.0, 0.0);
    }
    let hit_result = world.hit(ray, 0.001, INFINITY);
    if hit_result.is_some()
    {
        let hit_result = hit_result.unwrap();
        let scatter_result = hit_result.material().scatter(ray, &hit_result);
        if scatter_result.is_none()
        {
            return FColor::make_new(0.0, 0.0, 0.0);
        }
        let (attenuation, scattered_ray) = scatter_result.unwrap();
        return attenuation * ray_color(&scattered_ray, world, depth - 1);
    }
    let dir = ray.direction().get_normalized();
    let y = 0.5 * dir.y() + 0.5;
    (1.0 - y) * FColor::new_color(1.0, 1.0, 1.0) + y * FColor::new_color(0.5, 0.7, 1.0)
}

fn render_slice(camera: &Camera, world: &HittableList, start: usize, slice: &mut [Vec<FColor>])
{
    // Setup Deterministic Random Genrator:
    let mut rand_gen = SimpleDeterministicRandomGenerator::new();

    let mut u: f64;
    let mut v: f64;
    let mut ray: Ray;
    let mut color = FColor::make_new(0.0, 0.0, 0.0);

    for height_iterator in 0..slice.len()
    {
        for width_iterator in 0..slice[height_iterator].len()
        {
            color.reset();
            for _sample_number in 0..NUM_SAMPLES
            {
                //println!("casting ray {}, {}, {}", width_iterator, start + height_iterator, sample_number);
                u = 2.0 * (width_iterator as f64 + rand_gen.rand()) / RENDER_WIDTH as f64 - 1.0;
                v = 2.0 * ((start + height_iterator) as f64 + rand_gen.rand()) / RENDER_HEIGHT as f64 - 1.0;

                ray = camera.get_ray(u, v);

                color += &ray_color(&ray, world, MAX_CHILD_RAYS);
            }
            slice[height_iterator][width_iterator] = (&color / NUM_SAMPLES as f64).sqrt();
        }
    }
}

fn make_random_scene() -> Box<HittableList>
{
    let mut world = Box::new(HittableList::default());

    let ground_material = Arc::new(Lambertian::make_new(FColor::make_new(0.5, 0.5, 0.5)));
    let ground_sphere = Arc::new(Sphere::make_new(Point::make_new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    world.add(ground_sphere);

    let mut rand_gen = SimpleDeterministicRandomGenerator::new();

    // Add some random small balls
    for i in -11..11
    {
        for j in -11..11
        {
            let material_decider = rand_gen.rand();
            let center = Point::make_new(i as f64 + 0.9 * rand_gen.rand(), 0.2, j as f64+ 0.9 * rand_gen.rand());

            if (center - Point::make_new(4.0, 0.2, 0.0)).length() > 0.9
            {
                if material_decider < 0.5 // 50% chance for lambertian ball
                {
                    let albedo = FColor::rand();
                    let material = Arc::new(Lambertian::make_new(albedo));
                    let sphere = Arc::new(Sphere::make_new(center, 0.2, material));
                    world.add(sphere);
                }
                else if material_decider < 0.9 // 40% chance for metal ball
                {
                    let albedo = FColor::rand();
                    let roughness = rand_gen.rand();
                    let material = Arc::new(Metal::make_new(albedo, roughness));
                    let sphere = Arc::new(Sphere::make_new(center, 0.2, material));
                    world.add(sphere);
                }
                else // 10% chance for glass ball
                {
                    let material = Arc::new(Dielectric::make_new(1.5));
                    let sphere = Arc::new(Sphere::make_new(center, 0.2, material.clone()));
                    world.add(sphere);
                    if rand_gen.rand() < 0.3 // 30% for hollow glass ball
                    {
                        let inner_bubble = Arc::new(Sphere::make_new(center, -0.15, material));
                        world.add(inner_bubble);
                    }
                }
            }
        }
    }
    // add three big balls, one for each material
    let glass = Arc::new(Dielectric::make_new(1.5));
    let glass_ball = Arc::new(Sphere::make_new(Point::make_new(0.0, 1.0, 0.0), 1.0, glass));
    world.add(glass_ball);
    
    let diffuse = Arc::new(Lambertian::make_new(FColor::rand()));
    let diffuse_ball = Arc::new(Sphere::make_new(Point::make_new(-4.0, 1.0, 0.0), 1.0, diffuse));
    world.add(diffuse_ball);

    let metal = Arc::new(Metal::make_new(FColor::rand(), 0.5));
    let metal_ball = Arc::new(Sphere::make_new(Point::make_new(4.0, 1.0, 0.0), 1.0, metal));
    world.add(metal_ball);

    world
}

fn make_simple_scene() -> Box<HittableList>
{
    let mut world = Box::new(HittableList::default());
    
    // Materials:
    let material_ground = Arc::new(Lambertian::make_new(FColor::make_new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::make_new(FColor::make_new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::make_new(1.5));
    let material_right  = Arc::new(Metal::make_new(FColor::make_new(0.8, 0.6, 0.2), 0.0));

    // Spheres:
    let ground_sphere = Arc::new(Sphere::make_new(Point::make_new(0.0, -100.5, -1.0), 100.0, material_ground));
    let center_sphere = Arc::new(Sphere::make_new(Point::make_new(0.0, 0.0, -1.0), 0.5, material_center));
    let left_sphere = Arc::new(Sphere::make_new(Point::make_new(-1.0, 0.0, -1.0), 0.5, material_left.clone()));
    let left_sphere_inner = Arc::new(Sphere::make_new(Point::make_new(-1.0, 0.0, -1.0), -0.45, material_left));
    let right_sphere = Arc::new(Sphere::make_new(Point::make_new(1.0, 0.0, -1.0), 0.5, material_right));
    world.add(ground_sphere);
    world.add(center_sphere);
    world.add(left_sphere);
    world.add(left_sphere_inner);
    world.add(right_sphere);
    
    world
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
    let pos = Point::make_new(13.0, 2.0, 3.0);
    let lookat = Point::make_new(0.0, 0.0, 0.0);
    let world_up = Vect::make_new(0.0, 1.0, 0.0);
    let depth_of_field = (pos - lookat).length();
    let aperture = 0.1;
    let camera = Camera::new(pos, lookat, world_up, 20.0,  ASPECT_RATIO, aperture, depth_of_field);

    // Setup Frame Buffer (a.k.a. Render Target)
    let mut fb = FrameBuffer::<RENDER_WIDTH, RENDER_HEIGHT>::new();

    // Setup World:
    //let world = make_random_scene();
    let world = make_random_scene();

    let num_available_threads = available_parallelism().unwrap().get() * 2;
    let stride = RENDER_HEIGHT / num_available_threads;
    
    thread::scope(|s|
        {
            let camera = &camera;
            let world = &world;
            let mut slice = &mut fb.pixels[..];
            for thread_id in 0..num_available_threads-1
            {
                let (first_slice, rest_slice) = slice.split_at_mut(stride);
                Builder::new().spawn_scoped(s, move || render_slice(camera, world, thread_id * stride, first_slice)).unwrap();
                slice = rest_slice;
            }
            Builder::new().spawn_scoped(s, move || render_slice(camera, world, (num_available_threads - 1) * stride, slice)).unwrap();
        }
    );
    
    fb.write_to_file(&mut file)?;

    Ok(())
}
