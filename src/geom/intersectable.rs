use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use nalgebra::{Unit, Vector3};

use crate::geom::ray::Ray;
use crate::lighting::material::Material;

pub struct Intersection<'o,'r> {
    point : Vector3<f64>,
    dist : f64,
    normal : Unit<Vector3<f64>>,
    material : &'o Box<dyn Material>,
    ray_in : &'r Ray
}

impl<'o,'r> Intersection<'o,'r> {
    pub fn new(point : Vector3<f64>, dist: f64, normal : Unit<Vector3<f64>>, material : &'o Box<dyn Material>, ray_in : &'r Ray) -> Self {
        Intersection { point: point, dist: dist, normal: normal, material : material, ray_in }

    }

    pub fn normal(&self) -> Unit<Vector3<f64>> {
        self.normal
    }

    pub fn point(&self) -> Vector3<f64> {
        self.point
    }

    pub fn material(&self) -> &'o Box<dyn Material> {
        self.material
    }

    pub fn ray_in(&self) -> &'r Ray {
        self.ray_in
    }

}

pub trait Intersectable : Send + Sync {
    // It might be more efficient to pass in a &mut Option<Intersectoin>, but that's ugly.
    fn intersect<'o,'r>(&'o self, ray : &'r Ray, dist_min : f64, dist_max : f64) -> Option<Intersection<'o,'r>>;
}

impl<T : Intersectable> Intersectable for Vec<T> {
    fn intersect<'o,'r>(&'o self, ray : &'r Ray, dist_min : f64, dist_max : f64) -> Option<Intersection<'o,'r>> {
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
impl Intersectable for Arc<dyn Intersectable> {
    fn intersect<'o,'r>(&'o self, ray : &'r Ray, dist_min : f64, dist_max : f64) -> Option<Intersection<'o,'r>> {
        (**self).intersect(ray,dist_min,dist_max)
    }
}