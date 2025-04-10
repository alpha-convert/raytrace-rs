use nalgebra::{Unit, UnitVector3, Vector3};

use crate::{intersectable::Intersectable, light::Light};

pub struct Scene {
        pub objects : Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn new(objects : Vec<Box<dyn Intersectable>>) -> Self {
        Scene { objects: objects }

    }

}
