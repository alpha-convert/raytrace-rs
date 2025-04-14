use nalgebra::Vector2;

use super::Texture;

use std::sync::Arc;

pub struct ScaleTex {
    scale_u : f64,
    scale_v : f64,
    tex : Arc<dyn Texture>
}

impl Texture for ScaleTex {
    fn color_at(&self, uv: &nalgebra::Vector2<f64>, xyz: &nalgebra::Vector3<f64>) -> crate::lighting::color::Color {
        let uv2 = Vector2::new(uv.x / self.scale_u, uv.y / self.scale_v);
        self.tex.color_at(&uv2, xyz)
    }
}