use nalgebra::{Unit, Vector3};

use crate::{intersectable::{Intersectable, Intersection}, ray::Ray};

pub struct Plane {
    pt : Vector3<f64>,
    normal : Unit<Vector3<f64>>
}

impl Plane {
    pub fn new(center : Vector3<f64>, normal : Unit<Vector3<f64>>) -> Self {
        Plane { pt: center, normal: normal }
    }
}

impl Intersectable for Plane {
    fn intersect<'r>(&self, ray : &'r Ray) -> Option<Intersection<'r>> {
        let denom = self.normal.dot(ray.dir());
        if denom.abs() > 0.0 {
            let t = (self.pt - ray.origin()).dot(&self.normal) / denom;
            if t >= 0.0 {
                let point = ray.origin() + ray.dir().scale(t);
                return Some(Intersection::new(point,t,ray,self.normal))
            }
        }
        None
    }
}