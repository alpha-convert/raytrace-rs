use aabb::AABB;
use intersection::Intersection;
use interval::Interval;
use ray::Ray;

pub mod aabb;
pub mod axis;
pub mod cube;
pub mod interval;
pub mod plane;
pub mod quad;
pub mod ray;
pub mod sphere;
pub mod translation;
pub mod intersection;
pub mod bvh;

pub trait Geom: Send + Sync {
    // It might be more efficient to pass in a &mut Option<Intersectoin>, but that's ugly.
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>>;
    fn bbox(&self) -> &AABB;
}

