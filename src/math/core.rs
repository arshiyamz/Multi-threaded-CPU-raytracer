// Core math constants and functions

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub fn degrees_to_radians(degrees: f64) -> f64
{
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64
{
    radians * 180.0 / PI
}