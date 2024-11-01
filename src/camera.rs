use crate::{hittable::{HitRecord, Hittable}, util::{color::{print_color, Color}, interval::Interval, ray::Ray, vec::{Point3, Vec3}}};

pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: i32,
  image_height: i32,
  center: Point3,
  pixel_origin: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
          aspect_ratio: 1.0, 
          image_width: 100, 
          image_height: Default::default(), 
          center: Default::default(), 
          pixel_origin: Default::default(), 
          pixel_delta_u: Default::default(), 
          pixel_delta_v: Default::default() 
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
            let pix_center = self.pixel_origin + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
            let ray = Ray::new(self.center, pix_center - self.center);

            let pcol = self.ray_color(&ray, world);
            print_color(&pcol);
        }
    }
  }

  fn initialize(&mut self) {

    self.image_height = (self.image_width as f64 /self.aspect_ratio as f64) as i32;
    self.image_height = if self.image_height < 1 {1} else {self.image_height};

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

  fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, Interval::new(0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::from(1));
    }

    let a = 0.5 * (r.direction().to_normal().y() + 1.0);
    return (1.0 - a) * Color::new(1, 1, 1) + a * Color::new(0.5, 0.7, 1.0);
  }
}