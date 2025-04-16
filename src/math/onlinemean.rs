use nalgebra::{Vector, Vector3};

#[derive(Debug)]
pub struct OnlineMean {
    count: usize,
    mean: Vector3<f64>,
    conv: f64,
}

impl OnlineMean {
    pub fn new() -> Self {
        OnlineMean {
            count: 0,
            mean: Vector3::zeros(),
            conv: f64::MAX,
        }
    }

    pub fn add_sample(&mut self, sample: Vector3<f64>) {
        self.count += 1;
        if self.count == 1 {
            self.mean = sample;
        }

        let delta = sample - self.mean;
        let old_mean = self.mean;
        self.mean = self.mean + delta / (self.count as f64);
        self.conv = (old_mean - self.mean).magnitude_squared()
    }

    pub fn mean(&self) -> Vector3<f64> {
        self.mean
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn convergence_delta(&self) -> f64 {
        self.conv
    }
}

// // Initialize with first sample
// struct OnlineVariance {
//     int count = 0;
//     Color mean;
//     Color M2;  // Second moment about the mean
//     float variance = 0.0f;

//     // Update with a new sample
//     void addSample(const Color& sample) {

//     // Get current variance estimate
//     float getVariance() const {
//         return variance;
//     }
// };
