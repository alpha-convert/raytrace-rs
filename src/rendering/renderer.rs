use nalgebra::{Unit, Vector3};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// use itertools::Itertools;
use crate::{
    geom::Geom,
    lighting::color::Color,
    math::{interval::Interval, ray::Ray, welford::OnlineMean},
};

use super::{camera::Camera, par_buffer::{BufRow, ParBuffer}, scene::Scene};

pub struct Renderer {
    //Metadata
    recursion_depth: u64,

    //Canvas data
    window_width: usize,
    window_height: usize,

    //adaptive AA data
    samples_per_batch: u64,
    conv_cutoff : f64
}

impl Renderer {
    pub fn new(
        recursion_depth: u64,
        window_width: usize,
        window_height: usize,
        samples_per_batch : u64,
        conv_cutoff : f64
    ) -> Self {

        Renderer {
            recursion_depth: recursion_depth,
            window_width: window_width,
            window_height: window_height,
            samples_per_batch,
            conv_cutoff
        }
    }

    fn sample_uv() -> (f64, f64) {
        let mut rng = rand::rng();
        let du: f64 = rng.random::<f64>() - 0.5;
        let dv: f64 = rng.random::<f64>() - 0.5;
        assert!(-0.5 <= du && du <= 0.5);
        assert!(-0.5 <= dv && dv <= 0.5);
        (du, dv)
    }

    pub fn render(&self, camera: &Camera, scene: &Scene) -> ParBuffer {
        let buffer = ParBuffer::new(self.window_height as usize, self.window_width as usize);

        (0..self.window_height).into_par_iter().for_each(|y_idx| {
            let mut row = buffer.lock_row(y_idx as usize);
            for x_idx in 0..self.window_width {
                self.render_px(&mut row,camera,scene,x_idx,y_idx);
            }
        });

        buffer
    }

    /// Adaptive rendering. Estimate the pixel color online with welfords algorithm.
    fn render_px(&self, row : &mut BufRow<'_>, camera : &Camera, scene : &Scene, x_idx : usize, y_idx : usize){

        let mut estimator = OnlineMean::new();

        while estimator.convergence_delta() > self.conv_cutoff {
            for _ in 0..self.samples_per_batch {
                let (du, dv) = Renderer::sample_uv();
                let ray = camera.ray_through(x_idx as f64 + du, y_idx as f64 + dv);

                let color = Self::trace(ray, scene, self.recursion_depth);
                estimator.add_sample(color.inner_vec());
            }
        }

        // dbg!(estimator.convergence_delta());

        let px_color = Color::from_vec(estimator.mean());

        row.set(x_idx, px_color);
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

    // fn shade(inter : &Intersection) -> Color {
    //     let r = inter.normal().x.abs();
    //     let g = inter.normal().y.abs();
    //     let b = inter.normal().z.abs();

    //     Color::new(r, g, b)
    // }
}
