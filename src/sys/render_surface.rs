use sdl2::{render::Canvas, video::Window};

use crate::lighting::color::{Color, GammaColor};

pub trait RenderSurface {
    fn draw_point(&mut self,x : u64, y : u64, color : GammaColor);
}

impl RenderSurface for Canvas<Window> {
    fn draw_point(&mut self,x : u64, y : u64, color : GammaColor) {
        self.set_draw_color(color);
    }
}