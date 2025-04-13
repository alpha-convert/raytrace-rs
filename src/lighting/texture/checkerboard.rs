use crate::lighting::color::Color;

use super::Texture;

struct Checkerboard {
    albedo1: Color,
    albedo2: Color,
    size: f64,
}

impl Checkerboard {
    pub fn new(size: f64, albedo1: Color, albedo2: Color) -> Self {
        Checkerboard {
            albedo1: albedo1,
            albedo2: albedo2,
            size,
        }
    }
}

impl Texture for Checkerboard {
    fn color_at(&self, uv: &nalgebra::Vector2<f64>, xyz: &nalgebra::Vector3<f64>) -> Color {
        todo!()
    }
}
