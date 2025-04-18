use nalgebra::{Unit, Vector3};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

// use itertools::Itertools;
use crate::{
    geom::Geom,
    lighting::color::Color,
    math::{interval::Interval, onlinemean::OnlineMean, ray::Ray},
};

use super::{camera::Camera, par_buffer::ParBuffer, scene::Scene};

pub struct Renderer {
    //Metadata
    recursion_depth: u64,

    //Canvas data
    window_width: usize,
    window_height: usize,

    //adaptive AA data
    samples_per_batch: u64,
    conv_cutoff: f64,
}

impl Renderer {
    pub fn new(
        recursion_depth: u64,
        window_width: usize,
        window_height: usize,
        samples_per_batch: u64,
        conv_cutoff: f64,
    ) -> Self {
        Renderer {
            recursion_depth: recursion_depth,
            window_width: window_width,
            window_height: window_height,
            samples_per_batch,
            conv_cutoff,
        }
    }

    fn sample_uv() -> (f64, f64) {
        let mut rng = rand::rng();
        let du: f64 = rng.random::<f64>() - 0.5;
        let dv: f64 = rng.random::<f64>() - 0.5;
        (du, dv)
    }

    pub fn render(&self, camera: &Camera, scene: &Scene) -> ParBuffer {
        let mut buffer = ParBuffer::new(self.window_height as usize, self.window_width as usize);

        buffer.par_iter_mut().for_each(|((x_idx,y_idx), c)| {
            *c = self.render_px(camera, scene, x_idx, y_idx)
        });
        buffer
    }

    /// Adaptive rendering. Estimate the pixel color online, stop when it converges in L2 norm.
    fn render_px(&self, camera: &Camera, scene: &Scene, x_idx: usize, y_idx: usize) -> Color {
        let mut estimator = OnlineMean::new();

        while estimator.convergence_delta() > self.conv_cutoff {
            for _ in 0..self.samples_per_batch {
                let (du, dv) = Renderer::sample_uv();
                let ray = camera.ray_through(x_idx as f64 + du, y_idx as f64 + dv);

                let color = Self::trace(ray, scene, self.recursion_depth);
                estimator.add_sample(color.inner_vec());
            }
        }

        Color::from_vec(estimator.mean())
    }

    fn trace(ray: Ray, scene: &Scene, depth: u64) -> Color {
        if depth <= 0 {
            Color::black()
        } else {
            if let Some(inter) = scene.intersect(ray, Interval::new(0.001, f64::MAX)) {
                let emit = inter.material().emit(&inter.uv());
                match inter.material().scatter(&ray, &inter) {
                    None => emit,
                    Some(scatter) => {
                        return emit
                            + Self::trace(*scatter.ray(), scene, depth - 1) * scatter.color();
                    }
                }
            } else {
                scene.background_color()
            }
        }
    }
}
