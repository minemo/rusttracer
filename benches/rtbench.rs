use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use raytracer::{self, camera::Camera, hittable::HittableList, sphere::Sphere, util::vec::Point3};

pub fn simple_scene(c: &mut Criterion) {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 512;
    cam.samples_per_pixel = 1;

    let mut g = c.benchmark_group("Render");
    for i in 1..11 {
        cam.samples_per_pixel = i;
        g.bench_with_input(BenchmarkId::new("Samples", i), &i, |b, &_i| {
            b.iter(|| cam._render_quiet(black_box(&world)))
        });
    }
    g.finish();
}

criterion_group!(benches, simple_scene);
criterion_main!(benches);
