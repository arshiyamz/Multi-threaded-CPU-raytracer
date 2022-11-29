use std::marker;
use std::sync::Arc;

use super::hittable::*;
use super::vect::Vect;
use super::ray::Ray;
use super::point::Point;
use super::material::*;

pub struct Sphere
{
    pub center: Point,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere
{
    pub fn default() -> Self
    {
        Sphere
        {
            center: Point{data: [0.0, 0.0, 0.0]},
            radius: 1.0,
            material: Arc::new(Lambertian::default()),
        }
    }

    pub fn new(c: &Point, r: f64, material: Arc<dyn Material>) -> Self
    {
        Sphere
        {
            center: Point{data: c.data},
            radius: r,
            material,
        }
    }

    pub fn make_new(c: Point, r: f64, material: Arc<dyn Material>) -> Self
    {
        Sphere
        {
            center: c,
            radius: r,
            material,
        }
    }
}

impl Hittable for Sphere
{

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>
    {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let h = Vect::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let delta = h*h - a *c;
        if delta < 0.0
        {
            return None;
        }
        let sqrt_delta = delta.sqrt();

        let mut potential_hit = (-h - sqrt_delta) / a;
        if potential_hit < t_min || potential_hit > t_max
        {
            potential_hit = (-h + sqrt_delta) / a;
            if potential_hit < t_min || potential_hit > t_max
            {
                return None;
            }
        }

        let hit_point = r.at(potential_hit);
        let result = HitResult::make_new(
            hit_point, 
            (hit_point - &self.center) / self.radius,
            self.material.clone(),
            potential_hit,
            r,
        );
        Some(result)
    }
}

unsafe impl marker::Sync for Sphere {}