use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use util::vec::Point3;

mod camera;
mod hittable;
mod sphere;
mod util;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
