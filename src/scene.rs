use nalgebra::{Unit, UnitVector3, Vector3};

use crate::geom::{intersectable::{Intersectable, Intersection}, ray::Ray};

pub struct Scene {
        pub objects : Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn new(objects : Vec<Box<dyn Intersectable>>) -> Self {
        Scene { objects: objects }

    }
}

impl Intersectable for Scene {
    fn intersect<'o,'r>(&'o self, ray : &'r Ray, dist_min : f64, dist_max : f64) -> Option<Intersection<'o,'r>> {
        self.objects.intersect(ray, dist_min, dist_max)
    }
}