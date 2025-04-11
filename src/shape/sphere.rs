use nalgebra::{Unit, Vector3};
use sdl2::pixels::Color;

use crate::{geom::intersectable::{Intersectable, Intersection}, geom::ray::Ray};

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
    fn intersect(&self, ray: &Ray, dist_min : f64, dist_max : f64) -> Option<Intersection> {
        let oc = self.center - ray.origin();

        let h = ray.dir().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        
        let discriminant = h * h - c;
        
        if discriminant < 0.0 {
            return None;
        }

        let mut dist = h - discriminant.sqrt();

        if dist <= dist_min || dist >= dist_max {
            dist = h + discriminant.sqrt();
            if dist <= dist_min || dist >= dist_max {
                return None
            }
        }
        
        let point = ray.at(dist);
        let normal = Unit::new_normalize(point - self.center); // can also divide by the radius...

        Some(Intersection::new(point,dist,  normal))
    }
}