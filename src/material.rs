use std::fmt::Debug;

use crate::{
    hittable::HitRecord,
    util::{color::Color, ray::Ray, vec::Vec3},
};

pub trait Material {
    fn scatter(
        self: &Self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("Material")
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_normal();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;
        return true;
    }
}

impl Material for Metal {
    fn scatter(
        self: &Self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}
