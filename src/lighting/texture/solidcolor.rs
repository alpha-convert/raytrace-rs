use crate::lighting::color::Color;

use super::Texture;

pub struct SolidColor {
    albedo : Color
}

impl SolidColor {
    pub fn new(albedo : Color) -> Self {
        SolidColor { albedo: albedo }
    }
}

impl Texture for SolidColor {
    fn color_at(&self, uv : &nalgebra::Vector2<f64>, xyz : &nalgebra::Vector3<f64>) -> Color {
        self.albedo
    }
}