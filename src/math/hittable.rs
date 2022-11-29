use std::marker;
use std::sync::Arc;

use super::ray::Ray;
use super::point::Point;
use super::vect::Vect;
use super::material::*;

pub struct HitResult
{
    point: Point,
    normal: Vect,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitResult
{
    pub fn point(&self) -> &Point
    {
        &self.point
    }

    pub fn normal(&self) -> &Vect
    {
        &self.normal
    }

    pub fn material(&self) -> &dyn Material
    {
        self.material.as_ref()
    }

    pub fn t(&self) -> f64
    {
        self.t
    }

    pub fn front_face(&self) -> bool
    {
        self.front_face
    }

    pub fn default() -> Self
    {
        HitResult
        {
            point: Point::new(),
            normal: Vect::new(),
            material: Arc::new(Lambertian::default()),
            t: 0.0,
            front_face: true
        }
    }

    pub fn make_new(point: Point, mut normal: Vect, material: Arc<dyn Material>, t: f64, ray: &Ray) -> Self
    {
        let mut front_face = true;

        normal.normalize();

        if (Vect::dot(&normal, ray.direction())) > 0.0
        {
            front_face = false;
            normal = -normal;
        }

        HitResult
        {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable: marker::Sync
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}

pub struct HittableList
{
    pub hittables: Vec<Arc<dyn Hittable>>,
}

impl Hittable for HittableList
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>
    {
        let mut final_result = HitResult::default();
        let mut hit_anything = false;
        let mut closest_hit = t_max;

        for hittable in &self.hittables
        {
            let hit_result = hittable.hit(r, t_min, closest_hit);
            if hit_result.is_some()
            {
                let hit_result = hit_result.unwrap();
                hit_anything = true;
                closest_hit = hit_result.t;
                final_result = hit_result;
            }
        }

        if !hit_anything
        {
            return None;
        }

        Some(final_result)
    }
}

impl HittableList
{
    pub fn default() -> Self
    {
        HittableList
        {
            hittables: Vec::default(),
        }
    }

    pub fn new(hittable: Arc<dyn Hittable>) -> Self
    {
        HittableList
        {
            hittables: vec!(hittable),
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>)
    {
        self.hittables.push(hittable);
    }

    pub fn clear(&mut self)
    {
        self.hittables.clear();
    }
}

unsafe impl marker::Sync for HittableList {}