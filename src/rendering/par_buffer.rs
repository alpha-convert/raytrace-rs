use std::sync::{Mutex, MutexGuard};

use rayon::{
    iter::{
        Enumerate, IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::{IterMut, ParallelSliceMut},
};

use crate::lighting::color::Color;

use super::render_surface::RenderSurface;

/// A 2d matrix of colors, locked row by row.
pub struct ParBuffer {
    rows: usize,
    cols: usize,
    data: Box<[Color]>, //stored row-major
}

impl<'data> IntoParallelRefMutIterator<'data> for ParBuffer {
    type Iter = rayon::iter::Map<
        Enumerate<IterMut<'data, Color>>,
        impl Fn((usize, &'data mut Color)) -> ((usize, usize), &'data mut Color),
    >;

    type Item = ((usize, usize), &'data mut Color);

    fn par_iter_mut(&'data mut self) -> Self::Iter {
        self.data
            .as_parallel_slice_mut()
            .par_iter_mut()
            .enumerate()
            .map(|(i, c)| {
                let x = i % self.cols;
                let y = i / self.cols;
                ((x, y), c)
            })
    }
}

impl ParBuffer {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![Color::black(); rows * cols].into_boxed_slice();
        ParBuffer { rows, cols, data }
    }

    pub fn blit_to<T: RenderSurface>(&mut self, surf: &mut T) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                let color = self.data.get(y * self.cols + x).unwrap();
                surf.draw_point(x as u64, y as u64, color.gamma());
            }
        }
    }
}
