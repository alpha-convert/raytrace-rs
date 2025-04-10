use std::ops::{Deref, DerefMut};

use nalgebra::{Unit, Vector3};
use sdl2::pixels::Color;

use crate::ray::Ray;

pub struct Intersection<'r> {
    point : Vector3<f64>,
    dist : f64,
    ray : &'r Ray,
    normal : Unit<Vector3<f64>>
}

impl<'r> Intersection<'r> {
    pub fn new(point : Vector3<f64>, dist: f64, ray : &'r Ray, normal : Unit<Vector3<f64>>) -> Self {
        assert!(ray.origin() + ray.dir().scale(dist) == point);
        Intersection { point: point, dist: dist, ray: ray, normal: normal }

    }

    pub fn normal(&self) -> &Unit<Vector3<f64>> {
        &self.normal
    }

    pub fn dist(&self) -> f64 {
        self.dist
    }
}

pub trait Intersectable {
    fn intersect<'r>(&self, ray : &'r Ray) -> Option<Intersection<'r>>;
}

impl<T : Intersectable> Intersectable for &Vec<T> {
    fn intersect<'r>(&self, ray : &'r Ray) -> Option<Intersection<'r>> {
        let mut closest = None;
        for obj in self.iter() {
            match obj.intersect(&ray) {
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
    fn intersect<'r>(&self, ray : &'r Ray) -> Option<Intersection<'r>> {
        (**self).intersect(ray)
    }
}