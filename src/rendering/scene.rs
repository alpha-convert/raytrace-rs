use crate::{geom::{
    intersectable::{Intersectable, Intersection},
    interval::Interval,
    ray::Ray,
}, lighting::color::Color};

pub struct Scene {
    objects: Vec<Box<dyn Intersectable>>,
    background_color : Color,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Intersectable>>, background_color : Color) -> Self {
        Scene { objects: objects, background_color }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Intersectable for Scene {
    fn intersect<'r>(&'r self, ray: &'r Ray, i: Interval) -> Option<Intersection<'r>> {
        self.objects.iter().filter_map(|obj| { obj.intersect(ray, i)}).min_by(Intersection::dist_compare)
    }
}
