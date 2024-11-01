use num::{FromPrimitive, ToPrimitive};

use crate::hittable::{HitRecord, Hittable};
use crate::util::interval::Interval;
use crate::util::vec::{dot, Point3, Vec3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new<T>(center: Point3, radius: T) -> Self where T: ToPrimitive+FromPrimitive {
        Self {
            center: center,
            radius: radius.to_f64().unwrap().max(0.0),
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

        return true;
    }
}
