use std::ops::Mul;

use num::{FromPrimitive, ToPrimitive};

use crate::util::vec::{Point3, Vec3};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            dir: Default::default(),
        }
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    pub fn direction(&self) -> Point3 {
        return self.dir;
    }

    pub fn at<T>(&self, t: T) -> Point3
    where
        T: ToPrimitive+FromPrimitive,
        Vec3: Mul<T, Output = Vec3>,
    {
        return self.origin + self.dir * t;
    }
}

#[test]
fn create_default_ray() {
    let a = Ray::default();
    assert_eq!(
        a,
        Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))
    )
}

#[test]
fn get_ray_at() {
    let a = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 5.0, 2.0));
    assert_eq!(a.at(0.5), Point3::new(5.0, 2.5, 1.0))
}
