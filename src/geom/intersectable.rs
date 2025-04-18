use std::sync::Arc;

use crate::math::{interval::Interval, ray::Ray};

use super::{aabb::AABB, intersection::Intersection};

pub trait Intersectable: Send + Sync {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>>;

    // fn bbox(&self) -> AABB;
}

impl Intersectable for Arc<dyn Intersectable> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        (**self).intersect(ray, i)
    }
}
