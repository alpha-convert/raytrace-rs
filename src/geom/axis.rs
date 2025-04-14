use nalgebra::Vector3;
use rand::Rng;


#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn vec_idx(&self, v: Vector3<f64>) -> f64 {
        match self {
            Axis::X => v.x,
            Axis::Y => v.y,
            Axis::Z => v.z,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let a : u64 = rng.random::<u64>() % 3;
        if a == 0 {
            return Axis::X
        } else if a == 1 {
            return Axis::Y
        } else if a == 2 {
            return Axis::Z
        }
        panic!()
    }
}