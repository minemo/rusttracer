use core::f64;

use num::ToPrimitive;

#[derive(Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}

impl Interval {
    pub fn new<T, U>(min: T, max: U) -> Self
    where
        T: ToPrimitive,
        U: ToPrimitive,
    {
        Self {
            min: min.to_f64().unwrap(),
            max: max.to_f64().unwrap(),
        }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains<T>(&self, x: T) -> bool
    where
        T: ToPrimitive,
    {
        return self.min <= x.to_f64().unwrap() && x.to_f64().unwrap() <= self.max;
    }

    pub fn surrounds<T>(&self, x: T) -> bool
    where
        T: ToPrimitive,
    {
        return self.min < x.to_f64().unwrap() && x.to_f64().unwrap() < self.max;
    }

    pub fn clamp<T>(&self, x: T) -> f64
    where
        T: ToPrimitive,
    {
        let xf = x.to_f64().unwrap();
        return if xf < self.min {
            self.min
        } else if xf > self.max {
            self.max
        } else {
            xf
        };
    }

    pub const fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub const fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

#[test]
fn interval_contains() {
    let a = Interval::new(0, f64::INFINITY);
    assert!(a.contains(10));
    assert!(a.contains(13.37));
    assert!(!a.contains(-23));
    assert!(!a.contains(-0.1));
}

#[test]
fn interval_surrounds() {
    let a = Interval::new(0, 99.99);

    assert!(a.surrounds(10));
    assert!(a.surrounds(13.37));
    assert!(!a.surrounds(-23));
    assert!(!a.surrounds(110.1));
}

#[test]
fn interval_clamp() {
    let a = Interval::new(-10, 15.0);

    assert_eq!(a.clamp(10), 10.0);
    assert_eq!(a.clamp(20), 15.0);
    assert_eq!(a.clamp(-23), -10.0);
    assert_eq!(a.clamp(-5.2), -5.2);
}
