use std::f64;

use crate::util::{interval::Interval, vec};

pub type Color = vec::Vec3;

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    return 0.0;
}

pub fn print_color(c: &Color) {
    let intensity = Interval::new(0.000, 0.999);
    let mut r = c.x();
    let mut g = c.y();
    let mut b = c.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let rbyte = (256.0 * intensity.clamp(r)) as i16;
    let gbyte = (256.0 * intensity.clamp(g)) as i16;
    let bbyte = (256.0 * intensity.clamp(b)) as i16;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
