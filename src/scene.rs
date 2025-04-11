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

impl Intersectable for Scene {
    fn intersect(&self, ray : &crate::ray::Ray, dist_min : f64, dist_max : f64) -> Option<crate::intersectable::Intersection> {
        self.objects.intersect(ray, dist_min, dist_max)
    }
}