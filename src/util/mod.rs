use std::f64::consts::PI;

pub mod color;
pub mod ray;
pub mod vec;

pub fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}
