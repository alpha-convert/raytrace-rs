use nalgebra::{Unit, UnitVector3, Vector3};

#[derive(Debug)]
pub struct Ray {
    origin : Vector3<f64>,
    // Always normalized
    dir : Unit<Vector3<f64>>,
}

impl Ray {
    pub fn new(origin : Vector3<f64>, dir : UnitVector3<f64>) -> Self {
        Ray { origin : origin , dir: dir }
    }

    pub fn new_normalize(origin : Vector3<f64>, dir_unnormalized : Vector3<f64>) -> Self {
        Self::new(origin,Unit::new_normalize(dir_unnormalized))
    }
    
    pub fn through_points(from : Vector3<f64>, to : Vector3<f64>) -> Self {
        Ray::new(from,Unit::new_normalize(to - from))
    }

    pub fn origin(&self) -> &Vector3<f64> {
        &self.origin
    }

    pub fn dir(&self) -> &Vector3<f64> {
        &self.dir
    }

    pub fn at(&self, t : f64) -> Vector3<f64> {
        self.origin + self.dir.scale(t)
    }
}