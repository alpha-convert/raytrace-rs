use std::sync::{Mutex, MutexGuard};

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator};

use crate::lighting::color::Color;

use super::render_surface::RenderSurface;

/// A 2d matrix of colors, locked row by row.
pub struct ParBuffer {
    rows: usize,
    cols: usize,
    data: Vec<Vec<Color>>,
}

impl<'data> IntoParallelRefMutIterator<'data> for ParBuffer {
    type Iter = rayon::iter::Enumerate<<&'data mut Vec<Vec<Color>> as IntoParallelIterator>::Iter>;

    type Item = (usize, &'data mut Vec<Color>);

    fn par_iter_mut(&'data mut self) -> Self::Iter {
        self.data.par_iter_mut().enumerate()
    }
}

impl ParBuffer {
    pub fn new(rows: usize, cols: usize) -> Self {
        let single_row = vec![Color::black(); cols];
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(single_row.clone());
        }
        ParBuffer { rows, cols, data }
    }

    pub fn blit_to<T: RenderSurface>(&mut self, surf: &mut T) {
        for y in 0..self.rows {
            let row = self.data.get(y).unwrap();
            for x in 0..self.cols {
                let color = row.get(x).unwrap();
                surf.draw_point(x as u64, y as u64, color.gamma());
            }
        }
    }
}
