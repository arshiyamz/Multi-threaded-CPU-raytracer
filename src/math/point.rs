pub use super::vect::Vect as Point;

impl Point
{
    pub fn new_point(x: f64, y: f64, z: f64) -> Self
    {
        Point
        {
            data: [x, y, z]
        }
    }
}