use std::cmp::min_by;

use super::core::*;
use super::random::SimpleDeterministicRandomGenerator;
use super::ray::Ray;
use super::vect::Vect;
use super::hittable::HitResult;
use super::super::utils::color::FColor;

pub trait Material
{
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(FColor, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian
{
    albedo: FColor,
}

impl Material for Lambertian
{
    fn scatter(&self, _ray: &Ray, hit_result: &HitResult) -> Option<(FColor, Ray)>
    {
        let mut dir = Vect::random_in_hemisphere(hit_result.normal());
        if dir.is_zero()
        {
            dir = *hit_result.normal();
        }
        let ray = Ray::make_new(*hit_result.point(), dir);

        Some((self.albedo.clone(), ray))
    }
}

impl Lambertian
{
    pub const fn default() -> Self
    {
        Lambertian{ albedo: FColor{data: [0.4, 0.2, 0.6]} }
    }

    pub fn new(color: &FColor) -> Self
    {
        Lambertian{ albedo: color.clone() }
    }

    pub fn make_new(color: FColor) -> Self
    {
        Lambertian{ albedo: color }
    }
}

pub static DEFAULT_LAMBERTIAN: Lambertian = Lambertian::default();

#[derive(Clone)]
pub struct Metal
{
    albedo: FColor,
    roughness: f64
}

impl Material for Metal
{
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(FColor, Ray)>
    {
        let mut direction = Vect::reflect(ray.direction(), hit_result.normal());
        direction += self.roughness * Vect::random_in_hemisphere(&direction);
        let ray = Ray::make_new(*hit_result.point(), direction);

        if Vect::dot(ray.direction(), hit_result.normal()) <= 0.0
        {
            return None;
        }

        Some((self.albedo.clone(), ray))
    }
}

impl Metal
{
    pub const fn default() -> Self
    {
        Metal{ albedo: FColor{data: [0.4, 0.2, 0.6]}, roughness: 0.0 }
    }

    pub fn new(color: &FColor, roughness: f64) -> Self
    {
        Metal{ albedo: color.clone(), roughness: clamp(roughness, 0.0, 1.0) }
    }

    pub fn make_new(color: FColor, roughness: f64) -> Self
    {
        Metal{ albedo: color, roughness: clamp(roughness, 0.0, 1.0) }
    }
}

pub static DEFAULT_METAL: Metal = Metal::default();

#[derive(Clone)]
pub struct Dielectric
{
    index_of_refraction: f64,
}

impl Material for Dielectric
{
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<(FColor, Ray)>
    {
        let mut rand_gen = SimpleDeterministicRandomGenerator::new();
        let mut refraction_ratio = self.index_of_refraction;
        if hit_result.front_face()
        {
            refraction_ratio = 1.0 / refraction_ratio;
        }

        let cos_theta = min_by(Vect::dot(&-ray.direction(), &hit_result.normal()), 1.0, |a, b| a.partial_cmp(b).unwrap());
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut direction = Vect::refract(ray.direction(), hit_result.normal(), refraction_ratio);

        if (refraction_ratio * sin_theta > 1.0) || (Dielectric::reflectance(cos_theta, refraction_ratio) > rand_gen.rand())
        {
            direction = Vect::reflect(ray.direction(), hit_result.normal());
        }
        
        direction.normalize();

        Some((FColor::make_new(1.0, 1.0, 1.0), Ray::make_new(*hit_result.point(), direction)))
    }
}

impl Dielectric
{
    pub const fn default() -> Self
    {
        Dielectric{ index_of_refraction: 1.5 }
    }

    pub fn new(index_of_refraction: f64) -> Self
    {
        Dielectric{ index_of_refraction }
    }

    pub fn make_new(index_of_refraction: f64) -> Self
    {
        Dielectric{ index_of_refraction }
    }

    fn reflectance(cos: f64, ir: f64) -> f64
    {
        let mut r0 = (1.0 - ir) / (1.0 + ir);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

pub static DEFAULT_DIELECTRIC: Dielectric = Dielectric::default();

//============================================
//============================================
//===============Unit Tests===================
//============================================
//============================================

#[cfg(test)]
mod tests
{
    use super::*;
    use std::sync::Arc;

    #[test]
    fn creation_test()
    {
        let orig = Vect{data:[1f64, 2f64, 3f64]};
        let dir = Vect{data:[0f64, 0f64, 1f64]};
        let r = Ray::make_new(orig, dir);

        let p = Vect{data:[1f64, 2f64, 4f64]};
        let n = Vect{data:[0f64, -1f64, -1f64]};
        let mat = Arc::new(Dielectric::make_new(2.0));
        let hr = HitResult::make_new(p, n, mat, 1.0, &r);
        let (col, scat) = hr.material().scatter(&r, &hr).unwrap();
        println!("{}\n {}\n {}", col, scat.origin(), scat.direction());
        assert_eq!(2, 3);
    }
}