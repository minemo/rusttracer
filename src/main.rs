use std::f64::INFINITY;

use hittable::{HitRecord, Hittable, HittableList};
use sphere::Sphere;
use util::color::{print_color, Color};
use util::ray::Ray;
use util::vec::{Point3, Vec3};

mod util;
mod hittable;
mod sphere;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::from(1));
    }

    let a = 0.5 * (r.direction().to_normal().y() + 1.0);
    return (1.0 - a) * Color::new(1, 1, 1) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: i32 = 512;

    let mut img_height = (img_width as f64 / aspect_ratio) as i32;
    img_height = if img_height < 1 { 1 } else { img_height };

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

    // Camera
    let focal_length = 1.0;
    let view_height: f64 = 2.0;
    let view_width: f64 = view_height * (img_width as f64 / img_height as f64);
    let camera_center = Point3::default();

    let view_u = Vec3::new(view_width, 0, 0);
    let view_v = Vec3::new(0, -view_height, 0);

    let pixel_delta_u = view_u / img_width;
    let pixel_delta_v = view_v / img_height;

    let view_upper_left = camera_center - Vec3::new(0, 0, focal_length) - view_u / 2 - view_v / 2;
    let pix_origin = view_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{}\n{}\n255", img_width, img_height);
    for j in 0..img_height {
        for i in 0..img_width {
            let pix_center = pix_origin + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray = Ray::new(camera_center, pix_center - camera_center);

            let pcol = ray_color(&ray, &world);
            print_color(&pcol);
        }
    }
}
