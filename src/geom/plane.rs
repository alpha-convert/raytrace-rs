use nalgebra::{Unit, Vector3};

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
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(
        center: Vector3<f64>,
        normal: Unit<Vector3<f64>>,
        material: Box<dyn Material>,
    ) -> Self {
        Plane {
            pt: center,
            normal: normal,
            material,
        }
    }
}

impl Intersectable for Plane {
    fn intersect<'o>(&'o self, ray: Ray, i: Interval) -> Option<Intersection<'o>> {
        let denom = self.normal.dot(&ray.dir());
        if denom.abs() > 1e-8 {
            let t = (self.pt - ray.origin()).dot(&self.normal) / denom;
            if i.contains(t) {
                let point = ray.origin() + ray.dir().scale(t);
                return Some(Intersection::new(
                    point,
                    t,
                    self.normal,
                    &self.material,
                    ray,
                    todo!(),
                ));
            } else {
                // println!("Culled Plane: t={}, min={}, denom={}", t, dist_min, denom);
            }
        }
        None
    }
}
