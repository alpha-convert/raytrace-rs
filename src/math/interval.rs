#[derive(Debug, Copy, Clone, Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn length(&self) -> f64 {
        self.max - self.min
    }

    pub fn translate(&self, by: f64) -> Self {
        Interval {
            min: self.min + by,
            max: self.max + by,
        }
    }

    pub const UNIT: Interval = Interval { min: 0.0, max: 1.0 };

    pub fn new(min: f64, max: f64) -> Self {
        assert!(!min.is_nan());
        assert!(!max.is_nan());
        assert!(min <= max);
        Interval { min: min, max: max }
    }

    pub fn union(i0: Interval, i1: Interval) -> Self {
        let min = if i0.min <= i1.min { i0.min } else { i1.min };
        let max = if i0.max >= i1.max { i0.max } else { i1.max };
        Interval { min: min, max }
    }

    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn pad_by(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }
}
