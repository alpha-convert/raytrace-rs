use std::{default, sync::{Arc, Mutex, MutexGuard}};

use sdl2::{render::Canvas, video::Window};

use crate::lighting::color::Color;

pub struct ParBuffer {
    rows : usize,
    cols : usize,
    data : Vec<Mutex<Vec<Color>>>
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

    pub fn lock_row(&self, i : usize) -> MutexGuard<'_,Vec<Color>> {
        self.data.get(i).unwrap().lock().unwrap()
    }

    // This takes a &mut self because we want to make sure we only do this when we're done.
    pub fn blit(&mut self, canvas : &mut Canvas<Window>)
    {
        for y in 0..self.rows {
            let row = self.data.get(y).unwrap().lock().unwrap();
            for x in 0..self.cols {
                let color = row.get(x).unwrap();
                canvas.set_draw_color(*color);
                canvas.draw_point((x as i32,y as i32)).unwrap();
            }
        }
    }
}