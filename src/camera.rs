use rand::random;

use crate::{
    hittable::{HitRecord, Hittable},
    util::{
        color::{print_color, Color},
        interval::Interval,
        ray::Ray,
        vec::{Point3, Vec3},
    },
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: Default::default(),
            max_depth: 10,
            center: Default::default(),
            pixel_origin: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            pixel_samples_scale: Default::default(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{}\n{}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pcol = Color::default();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pcol += self.ray_color(&r, self.max_depth, world);
                }
                print_color(&(pcol * self.pixel_samples_scale));
            }
        }
    }

    pub fn _render_quiet(&mut self, world: &dyn Hittable) {
        self.initialize();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pcol = Color::default();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pcol += self.ray_color(&r, self.max_depth, world);
                }
            }
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio as f64) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let focal_length = 1.0;
        let view_height: f64 = 2.0;
        let view_width: f64 = view_height * (self.image_width as f64 / self.image_height as f64);

        let view_u = Vec3::new(view_width, 0, 0);
        let view_v = Vec3::new(0, -view_height, 0);

        self.pixel_delta_u = view_u / self.image_width;
        self.pixel_delta_v = view_v / self.image_height;

        let view_upper_left = self.center - Vec3::new(0, 0, focal_length) - view_u / 2 - view_v / 2;
        self.pixel_origin = view_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let px_sample = self.pixel_origin
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_dir = px_sample - ray_origin;

        return Ray::new(ray_origin, ray_dir);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0);
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0, f64::INFINITY), &mut rec) {
            let dir = rec.normal + Vec3::random_normal();
            return 0.5 * self.ray_color(&Ray::new(rec.p, dir), depth - 1, world);
        }

        let a = 0.5 * (r.direction().to_normal().y() + 1.0);
        return (1.0 - a) * Color::new(1, 1, 1) + a * Color::new(0.5, 0.7, 1.0);
    }
}
