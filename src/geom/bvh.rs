use std::marker::PhantomData;

use crate::{
    geom::{aabb::AABB, intersectable::Intersectable},
    math::{
        axis::Axis,
        interval::Interval,
        ray::{self, Ray},
    },
};

use super::{Geom, Geomable, bbox::Bbox, intersection::Intersection};

struct BVHLeaf<T> {
    bbox: AABB,
    inner: T,
}

impl<T> BVHLeaf<T> {
    pub fn new(inner: T) -> Self
    where
        T: Bbox,
    {
        let bb = inner.bbox();
        BVHLeaf {
            bbox: bb,
            inner: inner,
        }
    }

    pub fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<(Intersection<'r>)>
        where T : Intersectable
    {
        if self.bbox.intersect(&ray, i) {
            self.inner.intersect(ray, i)
        } else {
            None
        }
    }
}


    // fn bbox(&self) -> AABB {
    //     self.bbox.clone()
    // }

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
        T: Bbox,
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

    pub fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>>
        where T : Intersectable
    {
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
}

pub enum BVHTree<T> {
    Leaf(BVHLeaf<T>),
    Node(BVHNode<T>),
}

impl<T> BVHTree<T> {
    pub fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>>
        where T : Intersectable
    {
        match self {
            BVHTree::Leaf(leaf) => leaf.intersect(ray, i),
            BVHTree::Node(node) => node.intersect(ray, i)
        }
    }

    pub fn bbox(&self) -> AABB {
        match self {
            BVHTree::Leaf(bvhleaf) => bvhleaf.bbox.clone(),
            BVHTree::Node(bvhnode) => bvhnode.bbox_union.clone(),
        }
    }

    pub fn leaf(inner: T) -> Self
    where
        T: Bbox,
    {
        BVHTree::Leaf(BVHLeaf::new(inner))
    }

    pub fn node(left: BVHTree<T>, right: BVHTree<T>) -> Self
    where
        T: Bbox,
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

    fn construct(mut from: Vec<T>) -> Self
    where
        T: Bbox,
    {
        let n = from.len();
        assert!(n > 0);
        if n == 1 {
            let geom = from.remove(0);
            return BVHTree::Leaf(BVHLeaf::new(geom));
        } else if n == 2 {
            let gr = from.remove(1);
            let gl = from.remove(0);
            let left = BVHTree::leaf(gl);
            let right = BVHTree::leaf(gr);
            BVHTree::node(left, right)
        } else {
            let axis = Axis::random();

            from.sort_by(|this, that| AABB::axis_compare(axis, &this.bbox(), &that.bbox()));

            let mid = n / 2;

            let geoms_right = from.split_off(mid);
            let geoms_left = from;

            let left = Self::construct(geoms_left);
            let right = Self::construct(geoms_right);

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
        T: Bbox,
    {
        let t = BVHTree::construct(geoms);
        dbg!(t.depth());
        dbg!(t.size());
        BVH { tree: t }
    }

    pub fn top(&self) -> &BVHTree<T> {
        &self.tree
    }
        
    pub fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>>
        where T : Intersectable
    {
        self.tree.intersect(ray, i)
    }
}

// impl<T: Intersectable> Intersectable for BVH<T> {
//     

//     // fn bbox(&self) -> AABB {
//     //     self.tree.bbox()
//     // }
// }
