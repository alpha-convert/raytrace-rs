use std::marker::PhantomData;

use crate::{
    geom::{Geom, aabb::AABB},
    math::{
        axis::Axis,
        interval::Interval,
        ray::{self, Ray},
    },
};

use super::intersection::Intersection;

struct BVHLeaf<T> {
    bbox: AABB,
    geom: T,
}

impl<T> BVHLeaf<T> {
    pub fn new(geom: T) -> Self
    where
        T: Geom,
    {
        BVHLeaf {
            bbox: geom.bbox().clone(),
            geom: geom,
        }
    }
}

impl<T: Geom> Geom for BVHLeaf<T> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        if self.bbox.intersect(&ray, i) {
            self.geom.intersect(ray, i)
        } else {
            None
        }
    }

    fn intersect_packet<'r>(
            &'r self,
            raypacket: crate::math::raypacket::RayPacket,
            i: Interval,
        ) -> Vec<(usize, Intersection<'r>)> {
            self.geom.intersect_packet(raypacket, i)
    }

    fn bbox(&self) -> AABB {
        self.bbox.clone()
    }
}

struct BVHNode<T> {
    phantom: PhantomData<T>,
    bbox_union: AABB,
    bbox_left: AABB,
    bbox_right: AABB,
    left: Box<BVHTree<T>>,
    right: Box<BVHTree<T>>,
}

impl<T> BVHNode<T> {
    pub fn new(left: BVHTree<T>, right: BVHTree<T>) -> Self
    where
        T: Geom,
    {
        let bbl = left.bbox();
        let bbr = right.bbox();
        let bbu = AABB::union(&bbl, &bbr);

        BVHNode {
            phantom: PhantomData,
            bbox_union: bbu,
            bbox_left: bbl,
            bbox_right: bbr,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl<T: Geom> Geom for BVHNode<T> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        let in_left = self.bbox_left.intersect(&ray, i);
        let in_right = self.bbox_right.intersect(&ray, i);

        if in_left && in_right {
            let int_left = self.left.intersect(ray, i);
            let int_right = self.right.intersect(ray, i);

            [int_left, int_right]
                .into_iter()
                .filter_map(|o| o)
                .min_by(Intersection::dist_compare)
        } else if in_left {
            self.left.intersect(ray, i)
        } else if in_right {
            self.right.intersect(ray, i)
        } else {
            None
        }
    }

    fn bbox(&self) -> AABB {
        self.bbox_union.clone()
    }
}

enum BVHTree<T> {
    Leaf(BVHLeaf<T>),
    Node(BVHNode<T>),
}

impl<T: Geom> Geom for BVHTree<T> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        match self {
            BVHTree::Leaf(leaf) => leaf.intersect(ray, i),
            BVHTree::Node(node) => node.intersect(ray, i),
        }
    }

    fn intersect_packet<'r>(
            &'r self,
            raypacket: crate::math::raypacket::RayPacket,
            i: Interval,
        ) -> Vec<(usize, Intersection<'r>)> {
        match self {
            BVHTree::Leaf(leaf) => leaf.intersect_packet(raypacket, i),
            BVHTree::Node(node) => node.intersect_packet(raypacket, i)
        }
    }

    fn bbox(&self) -> AABB {
        match self {
            BVHTree::Leaf(leaf) => leaf.bbox(),
            BVHTree::Node(node) => node.bbox(),
        }
    }
}

impl<T> BVHTree<T> {
    pub fn leaf(geom: T) -> Self
    where
        T: Geom,
    {
        BVHTree::Leaf(BVHLeaf::new(geom))
    }

    pub fn node(left: BVHTree<T>, right: BVHTree<T>) -> Self
    where
        T: Geom,
    {
        BVHTree::Node(BVHNode::new(left, right))
    }

    pub fn depth(&self) -> usize {
        match self {
            BVHTree::Leaf(_) => 0,
            BVHTree::Node(bvhnode) => 1 + usize::max(bvhnode.left.depth(), bvhnode.right.depth()),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            BVHTree::Leaf(_) => 0,
            BVHTree::Node(bvhnode) => 1 + bvhnode.left.size() + bvhnode.right.size(),
        }
    }

    fn construct(mut geoms: Vec<T>) -> Self
    where
        T: Geom + Clone,
    {
        let n = geoms.len();
        assert!(n > 0);
        if n == 1 {
            let geom = geoms.remove(0);
            return BVHTree::Leaf(BVHLeaf::new(geom));
        } else if n == 2 {
            let gr = geoms.remove(1);
            let gl = geoms.remove(0);
            let left = BVHTree::leaf(gl);
            let right = BVHTree::leaf(gr);
            BVHTree::node(left, right)
        } else {
            let axis = Axis::random();

            geoms.sort_by(|this, that| AABB::axis_compare(axis, &this.bbox(), &that.bbox()));

            let mid = n / 2;
            let (geoms_left, geoms_right) = geoms.split_at_mut(mid);
            let left = Self::construct(geoms_left.to_vec());
            let right = Self::construct(geoms_right.to_vec());

            BVHTree::node(left, right)
        }
    }
}

pub struct BVH<T> {
    tree: BVHTree<T>,
}

impl<T> BVH<T> {
    pub fn construct(geoms: Vec<T>) -> Self
    where
        T: Geom + Clone,
    {
        let t = BVHTree::construct(geoms);
        dbg!(t.depth());
        dbg!(t.size());
        BVH { tree: t }
    }
}

impl<T: Geom> Geom for BVH<T> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        self.tree.intersect(ray, i)
    }

    fn bbox(&self) -> AABB {
        self.tree.bbox()
    }
}
