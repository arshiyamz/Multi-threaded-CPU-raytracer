use super::vect::Vect;
use super::point::Point;
use super::ray::Ray;
use super::core::*;

#[derive(Debug, PartialEq)]
pub struct Camera
{
    pub origin: Point,
    pub right: Vect,
    pub up: Vect,
    pub dof: f64,
    pub lens_radius: f64,
}

impl Camera
{
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * Camera::VIEWPORT_HEIGHT; 
    const FOCAL_LENGTH: f64 = 1.0;

    pub fn new(pos: Point, look_at: Point, mut world_up: Vect,
               vfov: f64, aspect_ratio: f64, aperture: f64, dof: f64) -> Self
    {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let forward = (look_at - pos).get_normalized();
        world_up.normalize();
        let right = Vect::cross(&forward, &world_up).get_normalized();
        let up = Vect::cross(&right, &forward).get_normalized();

        Camera
        {
            origin: pos,
            right: viewport_width * right / 2.0,
            up: viewport_height * up / 2.0,
            dof,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray
    {
        let rand = self.lens_radius * Vect::random_in_disk(1.0);
        let offset = self.right * rand.x() + self.up * rand.y();

        let forward = Vect::cross(&self.up, &self.right).get_normalized();
        Ray::make_new
        (
            self.origin + offset,
            self.dof * (u * &self.right + v * &self.up + forward) - offset,
        )
    }
}