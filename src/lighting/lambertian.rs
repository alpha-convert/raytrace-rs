use nalgebra::Unit;

use crate::{geom::ray::Ray, util::{self, is_small}};

use super::{color::Color, material::Material};

#[derive(Debug,Clone,Copy)]
pub struct Lambertian {
    albedo : Color
}

impl Lambertian {
    pub fn new(albedo : Color) -> Self {
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter<'o>(&'o self, inter : &crate::geom::intersectable::Intersection<'o,'_>) -> Option<(Color,Ray)> {
        let normal = inter.normal();
        let mut bounce_dir = *normal + *util::random_unit_vec3();
        if is_small(bounce_dir) {
            bounce_dir = *normal;
        }
        let bounce_ray = Ray::new(inter.point(),Unit::new_normalize(bounce_dir));
        Some((self.albedo,bounce_ray))
    }
}