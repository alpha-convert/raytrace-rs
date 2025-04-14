use nalgebra::Vector2;

use super::color::Color;

pub trait Texture: Sync + Send {
    fn color_at(&self, uv: &Vector2<f64>) -> Color;
}

pub mod checkerboard;
pub mod image;
pub mod scaletex;
pub mod solidcolor;
