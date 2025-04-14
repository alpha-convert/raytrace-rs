use std::sync::Arc;

use nalgebra::Vector3;
use sdl2::libc::close;

use crate::lighting::material::Material;

use super::{
    intersectable::{Intersectable, Intersection},
    interval::Interval,
    quad::Quad,
    ray::Ray,
};

pub struct Cube {
    faces: [Box<Quad>; 6],
}

impl Cube {
    pub fn new(c: Vector3<f64>, r: f64, material: Arc<dyn Material>) -> Self {
        todo!()
    }
}

impl Intersectable for Cube {
    fn intersect<'r>(&'r self, ray: &'r Ray, i: Interval) -> Option<Intersection<'r>> {
        (&self.faces)
            .into_iter()
            .filter_map(|face| face.intersect(ray, i))
            .min_by(Intersection::dist_compare)
    }
}
