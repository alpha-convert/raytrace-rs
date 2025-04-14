use crate::geom::{
    intersectable::{Intersectable, Intersection},
    interval::Interval,
    ray::Ray,
};

pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Intersectable>>) -> Self {
        Scene { objects: objects }
    }
}

impl Intersectable for Scene {
    fn intersect<'r>(&'r self, ray: &'r Ray, i: Interval) -> Option<Intersection<'r>> {
        self.objects.iter().filter_map(|obj| { obj.intersect(ray, i)}).min_by(Intersection::dist_compare)
    }
}
