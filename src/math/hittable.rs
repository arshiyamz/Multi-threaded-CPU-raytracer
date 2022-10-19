use super::ray::Ray;
use super::point::Point;
use super::vect::Vect;

#[derive(Debug)]
pub struct HitResult
{
    pub point: Point,
    pub normal: Vect,
    pub t: f64,
}

impl HitResult
{
    pub fn default() -> Self
    {
        HitResult
        {
            point: Point::new(),
            normal: Vect::new(),
            t: 0.0,
        }
    }
}

pub trait Hittable
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitResult);
}

pub struct HittableList<'a>
{
    pub hittables: Vec<&'a dyn Hittable>,
}

impl<'a> Hittable for HittableList<'a>
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitResult)
    {
        let mut final_result = HitResult::default();
        let mut hit_anything = false;
        let mut closest_hit = t_max;

        for hittable in &self.hittables
        {
            let (did_hit, hit_result) = hittable.hit(r, t_min, closest_hit);
            if did_hit
            {
                hit_anything = true;
                closest_hit = hit_result.t;
                final_result = hit_result;
            }
        }

        (hit_anything, final_result)
    }
}

impl<'a> HittableList<'a>
{
    pub fn default() -> Self
    {
        HittableList
        {
            hittables: Vec::default(),
        }
    }

    pub fn new(hittable: &'a dyn Hittable) -> Self
    {
        HittableList
        {
            hittables: vec!(hittable),
        }
    }

    pub fn add(&mut self, hittable: &'a dyn Hittable)
    {
        self.hittables.push(hittable);
    }

    pub fn clear(&mut self)
    {
        self.hittables.clear();
    }
}