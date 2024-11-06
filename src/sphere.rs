use std::rc::Rc;

use num::{FromPrimitive, ToPrimitive};

use crate::hittable::{HitRecord, Hittable};
use crate::material::{Lambertian, Material};
use crate::util::color::Color;
use crate::util::interval::Interval;
use crate::util::vec::{dot, Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>
}

impl Sphere {
    pub fn new<T>(center: Point3, radius: T, mat: Rc<dyn Material>) -> Self where T: ToPrimitive+FromPrimitive {
        Self {
            center: center,
            radius: radius.to_f64().unwrap().max(0.0),
            mat: mat
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::util::ray::Ray,
        ray_t: Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discr = h * h - a * c;

        if discr < 0.0 {
            return false;
        }

        let sqrtd = discr.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
