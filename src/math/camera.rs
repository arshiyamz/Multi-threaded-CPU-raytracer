use super::vect::Vect;
use super::point::Point;

#[derive(Debug, PartialEq)]
pub struct Camera
{
    pub origin: Point,
    pub right: Vect,
    pub up: Vect,
}

impl Camera
{
    pub fn new() -> Self
    {
        Camera
        {
            origin: Point::new_point(0.0, 0.0, 0.0),
            right: Vect::new_vect(1.0, 0.0, 0.0),
            up: Vect::new_vect(0.0, 1.0, 0.0)
        }
    }
}