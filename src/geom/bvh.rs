use std::sync::Arc;

use crate::geom::{Geom, aabb::AABB};

use super::{axis::Axis, intersection::Intersection, interval::Interval, ray::Ray};

pub enum BVH {
    BVHLeaf {
        bbox: AABB,
        geom: Arc<dyn Geom>,
    },
    BVHNode {
        bbox_union: Box<AABB>,
        bbox_left: Box<AABB>,
        bbox_right: Box<AABB>,
        left: Arc<BVH>,
        right: Arc<BVH>,
    },
}

impl BVH {
    pub fn construct(geoms: &mut [Arc<dyn Geom>]) -> Self {
        let n = geoms.len();
        assert!(n > 0);
        if n == 1 {
            let geom = &geoms[0];
            return BVH::BVHLeaf {
                bbox: geom.bbox().clone(),
                geom: geom.clone(),
            };
        } else if n == 2 {
            let gl = &geoms[0];
            let gr = &geoms[1];
            let bbl = gl.bbox();
            let bbr = gr.bbox();
            let left = Arc::new(BVH::BVHLeaf {
                bbox: gl.bbox().clone(),
                geom: gl.clone(),
            });
            let right = Arc::new(BVH::BVHLeaf {
                bbox: gr.bbox().clone(),
                geom: gr.clone(),
            });
            let bbu = AABB::union(&bbl, &bbr);
            return BVH::BVHNode {
                bbox_union: Box::new(bbu),
                bbox_left: Box::new(bbl.clone()),
                bbox_right: Box::new(bbr.clone()),
                left: left,
                right: right,
            };
        } else {
            let axis = Axis::random();

            geoms.sort_by(|this, that| AABB::axis_compare(axis, &this.bbox(), &that.bbox()));

            let mid = n / 2;
            let (geoms_left, geoms_right) = geoms.split_at_mut(mid);
            let bvh_left = Self::construct(geoms_left);
            let bvh_right = Self::construct(geoms_right);

            let bbl = bvh_left.bbox();
            let bbr = bvh_right.bbox();

            let bbu = AABB::union(&bbl, &bbr);

            BVH::BVHNode {
                bbox_union: Box::new(bbu),
                bbox_left: Box::new(bbl.clone()),
                bbox_right: Box::new(bbr.clone()),
                left: Arc::new(bvh_left),
                right: Arc::new(bvh_right),
            }
        }
    }
}

impl Geom for BVH {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        match self {
            BVH::BVHLeaf { bbox, geom } => {
                if bbox.intersect(&ray, i) {
                    geom.intersect(ray, i)
                } else {
                    None
                }
            }
            BVH::BVHNode {
                bbox_union: _,
                bbox_left,
                bbox_right,
                left,
                right,
            } => {
                let in_left = bbox_left.intersect(&ray, i);
                let in_right = bbox_right.intersect(&ray, i);

                if in_left && in_right {
                    let int_left = left.intersect(ray, i);
                    let int_right = right.intersect(ray, i);

                    [int_left, int_right]
                        .into_iter()
                        .filter_map(|o| o)
                        .min_by(Intersection::dist_compare)
                } else if in_left {
                    left.intersect(ray, i)
                } else if in_right {
                    right.intersect(ray, i)
                } else {
                    None
                }
            }
        }
    }

    fn bbox(&self) -> AABB {
        match self {
            BVH::BVHLeaf { bbox, geom: _ } => bbox.clone(),
            BVH::BVHNode {
                bbox_union,
                bbox_left,
                bbox_right,
                left,
                right,
            } => (**bbox_union).clone(),
        }
    }
}
