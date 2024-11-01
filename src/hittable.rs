use crate::util::{
    ray::Ray,
    vec::{dot, Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_facing: bool,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Default::default(),
            normal: Default::default(),
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
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut has_hit = false;
        let mut closest = ray_tmax;

        for o in &self.objects {
            if o.hit(r, ray_tmin, closest, &mut temp_rec) {
                has_hit = true;
                closest = temp_rec.t;
                *rec = temp_rec;
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

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
