use std::{borrow::Cow, mem::replace, sync::Arc};

use nalgebra::Vector3;

use crate::geom::{intersectable::{Intersectable, Intersection}, ray::Ray};

use super::interval::Interval;

pub struct Translation {
    trans : Vector3<f64>,
    inner : Arc<dyn Intersectable>
}

impl Translation {
    pub fn new(trans : Vector3<f64>, inner : Arc<dyn Intersectable>) -> Self {
        Translation { trans: trans, inner: inner }
    }
}

impl Intersectable for Translation {
    fn intersect<'r>(&'r self, ray : Ray, i : Interval) -> Option<Intersection<'r>> {
        let new_ray = Ray::new(ray.origin() - self.trans, ray.dir());
        match self.inner.intersect(new_ray, i) {
            None => None,
            Some(mut inter) => {
                //FIXME: shouldn't we also fixup the "ray_in" of the inter?
                *inter.point_mut() = inter.point() + self.trans;
                Some(inter)
            }
        }
    }
}