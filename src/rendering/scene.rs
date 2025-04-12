use std::sync::Arc;


use crate::geom::{intersectable::{Intersectable, Intersection}, interval::Interval, ray::Ray};

pub struct Scene {
        pub objects : Vec<Arc<dyn Intersectable>>,
}

impl Scene {
    pub fn new(objects : Vec<Arc<dyn Intersectable>>) -> Self {
        Scene { objects: objects }

    }
}

impl Intersectable for Scene {
    fn intersect<'o,'r>(&'o self, ray : Ray, i : Interval) -> Option<Intersection<'o>> {
        self.objects.intersect(ray, i)
    }

    fn intersect_bb(&self, _ : Ray, _ : Interval) -> bool {
        true
    }
}