use nalgebra::{Unit, Vector3};
use sdl2::pixels::Color;

use crate::{intersectable::{Intersectable, Intersection}, ray::Ray};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(center : Vector3<f64>, radius: f64, color: Color) -> Self {
        Sphere { center: center, radius: radius, color: color }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin() - self.center;

        let b = oc.dot(ray.dir());
        let c = oc.dot(&oc) - self.radius * self.radius;
        
        // Calculate discriminant
        let discriminant = b * b - c;
        
        if discriminant < 0.0 {
            return None;
        }
        
        // Calculate intersection distances
        let t1 = -b - discriminant.sqrt();
        let t2 = -b + discriminant.sqrt();
        
        let d = if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            return None
        };

        let point = ray.origin() + ray.dir().scale(d);
        let normal = Unit::new_normalize(point - self.center); // can also divide by the radius...

        Some(Intersection::new(point,d,  normal))
    }
}