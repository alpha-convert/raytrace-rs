use image::RgbImage;
use sdl2::{render::Canvas, video::Window};

use crate::lighting::color::GammaColor;

pub trait RenderSurface {
    fn draw_point(&mut self, x: u64, y: u64, color: GammaColor);
}

impl RenderSurface for Canvas<Window> {
    fn draw_point(&mut self, x: u64, y: u64, color: GammaColor) {
        self.set_draw_color(color);
        self.draw_point((x as i32, y as i32)).unwrap();
    }
}

impl RenderSurface for RgbImage {
    fn draw_point(&mut self, x: u64, y: u64, color: GammaColor) {
        self.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color.into())
    }
}
