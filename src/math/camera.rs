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
}

impl Camera
{
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * Camera::VIEWPORT_HEIGHT; 
    const FOCAL_LENGTH: f64 = 1.0;

    pub fn new() -> Self
    {
        Camera
        {
            origin: Point::make_new(0.0, 0.0, 0.0),
            right: Vect::make_new(1.0, 0.0, 0.0),
            up: Vect::make_new(0.0, 1.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray
    {
        Ray
        {
            origin: self.origin.clone(),
            direction: &self.origin + &(&(u * Camera::VIEWPORT_WIDTH * &self.right) + &(&(v * Camera::VIEWPORT_HEIGHT * &self.up) + &Vect::make_new(0.0, 0.0, -Camera::FOCAL_LENGTH))),
        }
    }
}