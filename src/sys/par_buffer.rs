use std::{default, sync::{Arc, Mutex, MutexGuard}};

use sdl2::{render::Canvas, video::Window};

use crate::lighting::color::Color;

use super::render_surface::RenderSurface;

pub struct ParBuffer {
    rows : usize,
    cols : usize,
    data : Vec<Mutex<Vec<Color>>>
}

pub struct BufRow<'a> {
    data : MutexGuard<'a,Vec<Color>>
}

impl<'a> BufRow<'a> {
    pub fn set(&mut self, j : usize, c : Color) {
        *(self.data.get_mut(j).unwrap()) = c;
    }
}

impl ParBuffer {

    pub fn new(rows : usize, cols : usize) -> Self
    {
        let single_row = vec![Color::black();cols];
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(Mutex::new(single_row.clone()));
        }
        ParBuffer { rows, cols, data }
    }

    pub fn lock_row(&self, i : usize) -> BufRow {
        BufRow { data : self.data.get(i).unwrap().lock().unwrap() }
    }

    pub fn blit_to<T : RenderSurface>(self, surf : &mut T)
    {
        for y in 0..self.rows {
            let row = self.data.get(y).unwrap().lock().unwrap();
            for x in 0..self.cols {
                let color = row.get(x).unwrap();
                surf.draw_point(x as u64, y as u64, color.gamma());
            }
        }
    }
}