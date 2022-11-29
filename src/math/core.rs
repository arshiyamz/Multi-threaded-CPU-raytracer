// Core math constants and functions

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.14159265358979323846264338327950288;

pub const FLOAT_MARGIN_OF_ERROR: f64 = 0.00000001;

pub fn degrees_to_radians(degrees: f64) -> f64
{
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64
{
    radians * 180.0 / PI
}

pub fn clamp(val: f64, min: f64, max: f64) -> f64
{
    let mut result = val;
    if result < min
    {
        result = min;
    }
    if result > max
    {
        result = max;
    }
    result
}