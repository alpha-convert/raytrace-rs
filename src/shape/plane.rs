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
    fn intersect(&self, ray : &Ray, dist_min : f64, dist_max : f64) -> Option<Intersection> {
        let denom = self.normal.dot(ray.dir());
        if denom.abs() > 1e-8 {
            let t = (self.pt - ray.origin()).dot(&self.normal) / denom;
            if t >= dist_min && t <= dist_max {
                let point = ray.origin() + ray.dir().scale(t);
                return Some(Intersection::new(point,t,self.normal))
            } else {
                // println!("Culled Plane: t={}, min={}, denom={}", t, dist_min, denom);
            }
        }
        None
    }
}