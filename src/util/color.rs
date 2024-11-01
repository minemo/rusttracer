
use crate::util::vec;

pub type Color = vec::Vec3;

pub fn print_color(c: &Color) {
    let [rbyte, gbyte, bbyte]: [i16; 3] = (*c * 255.999).v().map(|f| f as i16);

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
