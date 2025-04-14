use image::{ImageBuffer, ImageError, ImageReader, Rgb};
use nalgebra::{Vector2, Vector3};

use crate::lighting::color::Color;

use super::Texture;

pub struct Image {
    buf: ImageBuffer<Rgb<f32>, Vec<f32>>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn from_fname(fname: &str) -> Image {
        let i = ImageReader::open(fname).unwrap().decode().unwrap();

        let buf = i.into_rgb32f();
        let width = buf.width();
        let height = buf.height();
        Image { buf, width, height }
    }
}

impl Texture for Image {
    fn color_at(&self, uv: &Vector2<f64>) -> Color {
        assert!(uv.x < 1.0);
        assert!(uv.y < 1.0);
        assert!(0.0 <= uv.x);
        assert!(0.0 <= uv.y);
        let i = (uv.x * (self.width as f64)) as u32;
        let j = (uv.y * (self.height as f64)) as u32;
        let px = self.buf.get_pixel(i, j);
        Color::new(px[0] as f64, px[1] as f64, px[2] as f64)
    }
}
