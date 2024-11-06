use std::rc::Rc;

use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::material::{Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::util::color::Color;
use raytracer::util::vec::Point3;

fn main() {
    let mut world = HittableList::new();
    
    let ground_mat = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_mat = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let right_mat = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground_mat)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, center_mat)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left_mat)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right_mat)));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 768;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.render(&world);
}
