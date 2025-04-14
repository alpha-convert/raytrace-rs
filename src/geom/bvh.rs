use crate::geom::{aabb::AABB, intersectable::Geom};

use super::{intersectable::Intersection, interval::Interval, ray::Ray};

enum BVH {
    BVHLeaf {
        geom: Box<dyn Geom>,
    },
    BVHNode {
        bbox_left: AABB,
        bbox_right: AABB,
        left: Box<BVH>,
        right: Box<BVH>,
    },
}

impl BVH {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        match self {
            BVH::BVHLeaf { geom } => geom.intersect(ray, i),
            BVH::BVHNode {
                bbox_left,
                bbox_right,
                left,
                right,
            } => {
                if bbox_left.intersect(&ray, i) {
                    left.intersect(ray, i)
                } else if bbox_right.intersect(&ray, i) {
                    right.intersect(ray, i)
                } else {
                    None
                }
            }
        }
    }
}
