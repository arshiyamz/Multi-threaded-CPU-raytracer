use super::hittable::*;
use super::vect::Vect;
use super::ray::Ray;
use super::point::Point;

#[derive(Debug)]
pub struct Sphere
{
    pub center: Point,
    pub radius: f64,
}

impl Sphere
{
    pub fn default() -> Self
    {
        Sphere
        {
            center: Point{data: [0.0, 0.0, 0.0]},
            radius: 1.0,
        }
    }

    pub fn new(c: &Point, r: f64) -> Self
    {
        Sphere
        {
            center: Point{data: c.data},
            radius: r,
        }
    }

    pub fn make_new(c: Point, r: f64) -> Self
    {
        Sphere
        {
            center: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitResult)
    {
        let oc = &r.origin - &self.center;
        let a = r.direction().length_squared();
        let h = Vect::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let delta = h*h - a *c;
        if delta < 0.0
        {
            return (false, HitResult::default());
        }
        let sqrt_delta = delta.sqrt();

        let potential_hit = (-h - sqrt_delta) / a;
        if potential_hit < t_min || potential_hit > t_max
        {
            let potential_hit = (-h + sqrt_delta) / a;
            if potential_hit < t_min || potential_hit > t_max
            {
                return (false, HitResult::default());
            }
        }

        let hit_point = r.at(potential_hit);
        let result = HitResult{
            normal: &(&hit_point - &self.center) / self.radius,
            point: hit_point,
            t: potential_hit,
        };
        (true, result)
    }
}