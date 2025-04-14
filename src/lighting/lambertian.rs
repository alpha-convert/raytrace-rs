use std::sync::Arc;

use nalgebra::Unit;

use crate::{
    geom::ray::Ray,
    util::{self, is_small},
};

use super::{
    color::Color,
    material::{Material, Scatter},
    texture::{solidcolor::SolidColor, Texture},
};

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Lambertian { tex: tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, inter: &crate::geom::intersectable::Intersection) -> Option<Scatter> {
        let normal = inter.normal();
        let mut bounce_dir = *normal + *util::random_unit_vec3();
        if is_small(bounce_dir) {
            bounce_dir = *normal;
        }
        let bounce_ray = Ray::new(inter.point(), Unit::new_normalize(bounce_dir));
        let albedo = self.tex.color_at(&inter.uv());
        Some(Scatter::new(albedo,bounce_ray))
    }
}
