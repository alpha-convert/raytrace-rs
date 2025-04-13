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
    fn intersect(&self, ray: Ray, i: Interval) -> Option<Intersection> {
        let mut closest = None;
        for obj in self.objects.iter() {
            match obj.intersect(ray, i) {
                None => (),
                Some(inter) => match closest {
                    None => {
                        closest.replace(inter);
                    }
                    Some(ref inter2) => {
                        if inter.dist() < inter2.dist() {
                            closest.replace(inter);
                        }
                    }
                },
            }
        }
        closest
    }
}
