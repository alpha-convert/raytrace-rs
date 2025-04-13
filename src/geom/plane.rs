use std::sync::Arc;

use nalgebra::{Unit, UnitVector3, Vector2, Vector3};

use crate::{
    geom::{
        intersectable::{Intersectable, Intersection},
        ray::Ray,
    },
    lighting::material::Material,
};

use super::interval::Interval;

pub struct Plane {
    pt: Vector3<f64>,
    normal: Unit<Vector3<f64>>,
    u_hat: UnitVector3<f64>,
    v_hat: UnitVector3<f64>,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(
        center: Vector3<f64>,
        normal: UnitVector3<f64>,
        v_hat: UnitVector3<f64>,
        material: Arc<dyn Material>,
    ) -> Self {
        let u_hat = Unit::new_normalize(v_hat.cross(&normal));
        Plane {
            pt: center,
            normal,
            u_hat,
            v_hat,
            material,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray, i: Interval) -> Option<Intersection> {
        let denom = self.normal.dot(&ray.dir());
        if denom.abs() > 1e-8 {
            let t = (self.pt - ray.origin()).dot(&self.normal) / denom;
            if i.contains(t) {
                let ipoint = ray.origin() + ray.dir().scale(t);

                let to_ipoint = ipoint - self.pt;

                let u = to_ipoint.dot(&self.u_hat);
                let v = to_ipoint.dot(&self.v_hat);

                return Some(Intersection::new(
                    ipoint,
                    t,
                    self.normal,
                    self.material.clone(),
                    ray,
                    Vector2::new(u, v),
                ));
            } else {
                // println!("Culled Plane: t={}, min={}, denom={}", t, dist_min, denom);
            }
        }
        None
    }
}
