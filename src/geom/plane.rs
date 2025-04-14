use std::{borrow::Cow, sync::Arc};

use nalgebra::{Unit, UnitVector3, Vector2, Vector3};

use crate::{
    geom::{
        {Geom, intersection::Intersection},
        ray::Ray,
    },
    lighting::material::Material,
};

use super::interval::Interval;

pub struct Plane {
    pt: Vector3<f64>,
    normal: Unit<Vector3<f64>>,
    u_hat: Vector3<f64>,
    u_hat_mag_sqr: f64,
    v_hat: Vector3<f64>,
    v_hat_mag_sqr: f64,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(
        center: Vector3<f64>,
        normal: UnitVector3<f64>,
        u_hat: Vector3<f64>,
        v_hat: Vector3<f64>,
        material: Arc<dyn Material>,
    ) -> Self {
        assert!(u_hat.dot(&v_hat) < 1e-8);
        assert!(u_hat.dot(&normal) < 1e-8);
        Plane {
            pt: center,
            normal,
            u_hat,
            u_hat_mag_sqr: u_hat.magnitude_squared(),
            v_hat,
            v_hat_mag_sqr: v_hat.magnitude_squared(),
            material,
        }
    }
}

impl Geom for Plane {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        let denom = self.normal.dot(&ray.dir());
        if denom.abs() > 1e-8 {
            let t = (self.pt - ray.origin()).dot(&self.normal) / denom;
            if i.contains(t) {
                let ipoint = ray.origin() + ray.dir().scale(t);

                let to_ipoint = ipoint - self.pt;

                let mut u = (to_ipoint.dot(&self.u_hat) / self.u_hat_mag_sqr).fract();
                let mut v = (to_ipoint.dot(&self.v_hat) / self.v_hat_mag_sqr).fract();

                assert!(-1.0 < u);
                assert!(-1.0 < v);
                assert!(u < 1.0);
                assert!(v < 1.0);

                u = (1.0 + u) / 2.0;
                v = (1.0 + v) / 2.0;

                return Some(Intersection::new(
                    ipoint,
                    t,
                    self.normal,
                    self.material.as_ref(),
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
