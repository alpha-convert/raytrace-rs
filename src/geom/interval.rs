use std::cmp::min;

use nalgebra::Vector3;

#[derive(Debug,Copy,Clone)]
pub struct Interval {
    pub min : f64,
    pub max : f64
}


impl Interval {
    pub fn new(min : f64, max : f64) -> Self {
        Interval { min: min, max: max }
    }

    pub fn union(i0 : Interval, i1 : Interval) -> Self {
        let min = if i0.min <= i1.min { i0.min } else { i1.min };
        let max = if i0.max >= i1.max { i0.max } else { i1.max };
        Interval { min: min,max }
    }

    pub fn contains(&self, t : f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn pad_by(&self, delta : f64) -> Self {
        let padding = delta / 2.0;
        Interval { min: self.min - padding, max: self.max + padding }
    }

}
