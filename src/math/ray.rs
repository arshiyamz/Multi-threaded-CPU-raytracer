use super::point::Point;
use super::vect::Vect;

#[derive(Debug, PartialEq)]
pub struct Ray
{
    origin: Point,
    direction: Vect,
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
        Ray{origin: *p, direction: d.get_normalized()}
    }

    pub fn make_new(p: Point, d: Vect) -> Ray
    {
        Ray{origin: p, direction: d.get_normalized()}
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
        &self.origin + (t * &self.direction)
    }
}


//============================================
//============================================
//===============Unit Tests===================
//============================================
//============================================

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn creation_test()
    {
        let r = Ray::default();
        assert_eq!(r, Ray
                        {
                            origin: Point{data: [0f64, 0f64, 0f64]},
                            direction: Vect{data: [1f64, 0f64, 0f64]},
                        });

        let u = Vect{data:[1f64, 2f64, 3f64]};
        let v = Vect{data:[4f64, 5f64, 6f64]};
        let r = Ray::new(&u, &v);
        assert_eq!(r, Ray
                        {
                            origin: Point{data: [1f64, 2f64, 3f64]},
                            direction: Vect{data: [4f64, 5f64, 6f64]},
                        });
        let r = Ray::make_new(u, v);
        assert_eq!(r, Ray
            {
                origin: Point{data: [1f64, 2f64, 3f64]},
                direction: Vect{data: [4f64, 5f64, 6f64]},
            });
    }

    #[test]
    fn at_test()
    {
        let u = Vect{data:[1f64, 2f64, 3f64]};
        let v = Vect{data:[4f64, 5f64, 6f64]};
        let r = Ray::make_new(u, v);
        assert_eq!(r.at(2f64), Vect{data:[9f64, 12f64, 15f64]});
    }
}