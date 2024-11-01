use core::f64;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use sphere::Sphere;
use util::color::{print_color, Color};
use util::interval::Interval;
use util::ray::Ray;
use util::vec::{Point3, Vec3};

mod util;
mod hittable;
mod sphere;
mod camera;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 512;

    cam.render(&world);
    
}
