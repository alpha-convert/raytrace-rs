use nalgebra::Vector2;

use crate::geom::{intersection::Intersection, ray::Ray};

use super::color::Color;

pub struct Scatter {
    color: Color,
    ray: Ray,
}

impl Scatter {
    pub fn new(c: Color, r: Ray) -> Self {
        Scatter { color: c, ray: r }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, inter: &Intersection) -> Option<Scatter>;

    fn emit(&self, _: &Vector2<f64>) -> Color {
        Color::black()
    }
}
