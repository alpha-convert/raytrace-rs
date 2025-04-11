use std::ops::{Deref, DerefMut};

use nalgebra::{Unit, Vector3};
use sdl2::pixels::Color;

use crate::ray::Ray;

pub struct Intersection {
    point : Vector3<f64>,
    dist : f64,
    normal : Unit<Vector3<f64>>
}

impl Intersection {
    pub fn new(point : Vector3<f64>, dist: f64, normal : Unit<Vector3<f64>>) -> Self {
        Intersection { point: point, dist: dist, normal: normal }

    }

    pub fn normal(&self) -> &Unit<Vector3<f64>> {
        &self.normal
    }

    pub fn point(&self) -> &Vector3<f64> {
        &self.normal
    }

}

pub trait Intersectable {
    // It might be more efficient to pass in a &mut Option<Intersectoin>, but that's ugly.
    fn intersect(&self, ray : &Ray, dist_min : f64, dist_max : f64) -> Option<Intersection>;
}

impl<T : Intersectable> Intersectable for Vec<T> {
    fn intersect(&self, ray : &Ray, dist_min : f64, dist_max : f64) -> Option<Intersection> {
        let mut closest = None;
        for obj in self.iter() {
            match obj.intersect(&ray,dist_min,dist_max) {
                None => (),
                Some(inter) => 
                    match closest {
                        None => { closest.replace(inter); },
                        Some(ref inter2) => {
                            if inter.dist < inter2.dist {
                                closest.replace(inter);
                            }
                        }

                    }
            }
        }
        closest
    }
}

// TODO: for some reason I can't do this as <T : Intersectable> Box<T>... unclear why.
impl Intersectable for Box<dyn Intersectable> {
    fn intersect(&self, ray : &Ray, dist_min : f64, dist_max : f64) -> Option<Intersection> {
        (**self).intersect(ray,dist_min,dist_max)
    }
}