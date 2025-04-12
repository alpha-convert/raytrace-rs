use nalgebra::Vector3;

use super::{interval::Interval, ray::Ray};

#[derive(Debug,Clone, Copy)]
enum Axis {
    X,
    Y,
    Z
}

impl Axis {
    pub fn vec_idx(&self, v : Vector3<f64>) -> f64 {
        match self {
            Axis::X => v.x,
            Axis::Y => v.y,
            Axis::Z => v.z
        }

    }
}


// Axis-aligned bounding box represented by the intervals in space it covers.
pub struct AABB {
    x : Interval,
    y : Interval,
    z : Interval
}

impl AABB {
    fn new(x : Interval, y : Interval, z : Interval) -> Self {
        AABB { x: x, y: y, z: z }
    }

    pub fn from_points(v1 : Vector3<f64>, v2 : Vector3<f64>) -> Self {
        let x = if v1.x <= v2.x { Interval::new(v1.x, v2.x) } else { Interval::new(v2.x,v1.x) };
        let y = if v1.y <= v2.y { Interval::new(v1.y, v2.y) } else { Interval::new(v2.y,v1.y) };
        let z = if v1.z <= v2.z { Interval::new(v1.z, v2.z) } else { Interval::new(v2.z,v1.z) };
        AABB { x: x, y: y, z: z }
    }

    pub fn idx(&self, a : Axis) -> Interval {
        match a {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn intersect(&self, r : Ray, mut i : Interval) -> bool {
        for a in [Axis::X,Axis::Y,Axis::Z] {
            let axi = self.idx(a);
            let adinv = 1.0 / a.vec_idx(*r.dir());

            let t0 = (axi.min - a.vec_idx(r.origin())) * adinv;
            let t1 = (axi.max - a.vec_idx(r.origin())) * adinv;

            if t0 < t1 {
                if t0 > i.min { i.min = t0 }
                if t1 < i.max { i.max = t1 }
            } else {
                if t1 > i.min { i.min = t1 }
                if t0 < i.max { i.max = t0 }
            }

            if i.max <= i.min { return false }
        }
        true
    }

}