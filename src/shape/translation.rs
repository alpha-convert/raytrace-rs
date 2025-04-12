use std::sync::Arc;

use nalgebra::Vector3;

use crate::geom::{intersectable::{Intersectable, Intersection}, ray::Ray};

pub struct Translation {
    trans : Vector3<f64>,
    inner : Arc<dyn Intersectable>
}

impl Intersectable for Translation {
    fn intersect<'o,'r>(&'o self, ray : Ray, dist_min : f64, dist_max : f64) -> Option<Intersection<'o>> {
        let ray = Ray::new(ray.origin() - self.trans, ray.dir());
        match self.inner.intersect(ray, dist_min, dist_max) {
            None => None,
            Some(mut inter) => {
                *inter.point_mut() = inter.point() + self.trans;
                Some(inter)
            }
        }
    }
}