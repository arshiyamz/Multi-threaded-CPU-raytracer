use super::point::Point;
use super::vect::Vect;

#[derive(Debug, PartialEq)]
pub struct Ray
{
    pub origin: Point,
    pub direction: Vect,
}

impl Ray
{
    pub fn default() -> Ray
    {
        Ray
        {
            origin: Point{data: [0f64, 0f64, 0f64]},
            direction: Vect{data: [1f64, 0f64, 0f64]},
        }
    }

    pub fn new(p: &Point, d: &Vect) -> Ray
    {
        Ray{origin: p.clone(), direction: d.clone()}
    }

    pub fn make_new(p: Point, d: Vect) -> Ray
    {
        Ray{origin: p, direction: d}
    }

    pub fn origin(&self) -> &Point
    {
        &self.origin
    }

    pub fn direction(&self) -> &Vect
    {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point
    {
        &self.origin + &(t * &self.direction)
    }
}
