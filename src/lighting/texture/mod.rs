use nalgebra::{Vector2, Vector3};

use super::color::Color;

pub trait Texture: Sync + Send {
    fn color_at(&self, uv: &Vector2<f64>, xyz: &Vector3<f64>) -> Color;
}

pub mod checkerboard;
pub mod solidcolor;
pub mod image;
pub mod scaletex;