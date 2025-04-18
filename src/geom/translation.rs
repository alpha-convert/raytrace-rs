use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    geom::{intersectable::Intersectable, intersection::Intersection},
    math::{interval::Interval, ray::Ray},
};

use super::{aabb::AABB, bbox::Bbox, Geomable};

pub struct Translation<T> {
    trans: Vector3<f64>,
    inner: T,
}

impl<T> Translation<T> {
    pub fn new(trans: Vector3<f64>, inner: T) -> Self {
        Translation {
            trans: trans,
            inner: inner,
        }
    }
}

impl<T : Intersectable> Intersectable for Translation<T> {
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

    
}

impl<T : Geomable> Geomable for Translation<T> {
    fn into_geoms(self) -> impl Iterator<Item = super::Geom> {
        self.inner.into_geoms().map(move |g| { super::Geom::Trans(Box::new(Translation { trans : self.trans , inner: g }))})
    }

    

}

impl<T : Bbox> Bbox for Translation<T> {
    fn bbox(&self) -> AABB {
        self.inner.bbox().translate(self.trans)
    }
}
