use std::sync::Arc;

use crate::math::ray::Ray;

use super::{
    color::Color,
    material::Material,
    texture::{Texture, solidcolor::SolidColor},
};

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        DiffuseLight { tex: tex }
    }

    pub fn solid(c: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(c)))
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &Ray,
        _: &crate::geom::intersection::Intersection,
    ) -> Option<super::material::Scatter> {
        None
    }

    fn emit(&self, uv: &nalgebra::Vector2<f64>) -> Color {
        self.tex.color_at(&uv)
    }
}
