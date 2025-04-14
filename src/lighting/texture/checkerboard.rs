use std::sync::Arc;

use crate::lighting::color::Color;

use super::Texture;

pub struct Checkerboard {
    tex1: Arc<dyn Texture>,
    tex2: Arc<dyn Texture>,
    checker_size: f64,
}

impl Checkerboard {
    pub fn new(checker_size: f64, tex1: Arc<dyn Texture>, tex2: Arc<dyn Texture>) -> Self {
        Checkerboard {
            tex1: tex1,
            tex2: tex2,
            checker_size,
        }
    }
}

impl Texture for Checkerboard {
    fn color_at(&self, uv: &nalgebra::Vector2<f64>, xyz: &nalgebra::Vector3<f64>) -> Color {
        let x = (uv.x / self.checker_size).floor() as i64;
        let y = (uv.y / self.checker_size).floor() as i64;

        if (x + y) % 2 == 0 {
            self.tex1.color_at(uv, xyz)
        } else {
            self.tex2.color_at(uv, xyz)
        }
    }
}
