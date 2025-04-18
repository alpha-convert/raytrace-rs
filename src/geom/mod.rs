use std::sync::Arc;

use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::raypacket::RayPacket;
use aabb::AABB;
use intersection::Intersection;

pub mod aabb;
pub mod bvh;
pub mod cube;
pub mod intersection;
pub mod plane;
pub mod quad;
pub mod rotation;
pub mod scaling;
pub mod sphere;
pub mod translation;
pub mod triangle;
pub mod trimesh;
pub mod bvhcache;

pub trait Geom: Send + Sync {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>>;

    fn intersect_packet<'r>(
        &'r self,
        raypacket: RayPacket,
        i: Interval,
    ) -> Vec<(usize, Intersection<'r>)> {
        raypacket
            .into_iter()
            .enumerate()
            .filter_map(|(j, ray)| {
                let int = self.intersect(ray, i)?;
                Some((j, int))
            })
            .collect()
    }
    fn bbox(&self) -> AABB;
}

impl Geom for Arc<dyn Geom> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        (**self).intersect(ray, i)
    }

    fn intersect_packet<'r>(
        &'r self,
        raypacket: RayPacket,
        i: Interval,
    ) -> Vec<(usize, Intersection<'r>)> {
        (**self).intersect_packet(raypacket, i)
    }

    fn bbox(&self) -> AABB {
        (**self).bbox()
    }
}
