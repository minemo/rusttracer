use std::rc::Rc;

use crate::{material::{Lambertian, Material}, util::{
    color::Color, interval::Interval, ray::Ray, vec::{dot, Point3, Vec3}
}};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_facing: bool,
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Default::default(),
            normal: Default::default(),
            mat: Rc::new(Lambertian::new(Color::random())),
            t: Default::default(),
            front_facing: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_facing = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_facing {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: Default::default(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut has_hit = false;
        let mut closest = ray_t.max;

        for o in &self.objects {
            if o.hit(r, Interval::new(ray_t.min, closest), &mut temp_rec) {
                has_hit = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return has_hit;
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}
