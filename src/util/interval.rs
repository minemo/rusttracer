use core::f64;

use num::ToPrimitive;

pub struct Interval {
  pub min: f64,
  pub max: f64
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: f64::INFINITY, max: -f64::INFINITY }
    }
}

impl Interval {
  pub fn new<T,U>(min:T,max:U) -> Self where T: ToPrimitive,U: ToPrimitive {
    Self { min: min.to_f64().unwrap(), max: max.to_f64().unwrap() }
  }

  pub fn size(&self) -> f64{
    return self.max - self.min;
  }

  pub fn contains<T>(&self, x: T) -> bool where T: ToPrimitive {
    return self.min <= x.to_f64().unwrap() && x.to_f64().unwrap() <= self.max
  }

  pub fn surrounds<T>(&self, x: T) -> bool where T: ToPrimitive {
    return self.min < x.to_f64().unwrap() && x.to_f64().unwrap() < self.max;
  }

  pub fn clamp<T>(&self, x: T) -> f64 where T: ToPrimitive {
    let xf = x.to_f64().unwrap();
    return if xf<self.min {self.min} else if xf>self.max {self.max} else {xf};
  }

  pub const fn empty() -> Self {
    Self { min: f64::INFINITY, max: -f64::INFINITY }
  }

  pub const fn universe() -> Self {
    Self { min: -f64::INFINITY, max: f64::INFINITY }
  }

}

