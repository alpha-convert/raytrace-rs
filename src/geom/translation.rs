use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    geom::{Geom, intersection::Intersection},
    math::{interval::Interval, ray::Ray},
};

use super::aabb::AABB;

pub struct Translation {
    trans: Vector3<f64>,
    inner: Arc<dyn Geom>,
}

impl Translation {
    pub fn new(trans: Vector3<f64>, inner: Arc<dyn Geom>) -> Self {
        Translation {
            trans: trans,
            inner: inner,
        }
    }
}

impl Geom for Translation {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        let new_ray = Ray::new(ray.origin() - self.trans, ray.dir());
        match self.inner.intersect(new_ray, i) {
            None => None,
            Some(mut inter) => {
                *inter.point_mut() = inter.point() + self.trans;
                Some(inter)
            }
        }
    }

    fn bbox(&self) -> AABB {
        self.inner.bbox().translate(self.trans)
    }
}
