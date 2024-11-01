use rand::random;

pub mod color;
pub mod interval;
pub mod ray;
pub mod vec;

pub fn random_range(min: f64, max: f64) -> f64 {
    return min + (max - min) * random::<f64>();
}
