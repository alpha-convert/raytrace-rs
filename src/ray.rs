use nalgebra::{Unit, UnitVector3, Vector3};

pub struct Ray {
    origin : Vector3<f64>,
    // Always normalized
    dir : Unit<Vector3<f64>>,
}

impl Ray {
    fn new(origin : Vector3<f64>, dir : UnitVector3<f64>) -> Self {
        Ray { origin : origin , dir: dir }
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
}