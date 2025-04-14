use std::{borrow::Cow, sync::Arc};

use nalgebra::{Unit, UnitVector3, Vector2, Vector3};

use crate::lighting::material::Material;

use super::{
    aabb::AABB,
    intersectable::{Geom, Intersection},
    interval::Interval,
    ray::Ray,
};

pub struct Quad {
    q: Vector3<f64>,
    u_hat: Vector3<f64>,
    v_hat: Vector3<f64>,

    normal: UnitVector3<f64>,
    d: f64,
    w: Vector3<f64>,

    bbox: AABB,
    mat: Arc<dyn Material>,
}

impl Quad {
    pub fn new(
        q: Vector3<f64>,
        u_hat: Vector3<f64>,
        v_hat: Vector3<f64>,
        mat: Arc<dyn Material>,
    ) -> Self {
        let bb1 = AABB::from_points(q, q + u_hat + v_hat);
        let bb2 = AABB::from_points(q + u_hat, q + v_hat);
        let bbox = AABB::union(bb1, bb2);

        let n = u_hat.cross(&v_hat);
        let normal = Unit::new_normalize(n);
        let d = q.dot(&normal);

        let w = n.scale(1.0 / n.dot(&n));

        Quad {
            q,
            u_hat,
            v_hat,
            bbox,
            mat,
            normal,
            d,
            w,
        }
    }
}

impl Geom for Quad {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        if !self.bbox.intersect(&ray, i) {
            return None;
        }

        let denom = self.normal.dot(&ray.dir());

        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot(&ray.origin())) / denom;

        if !i.contains(t) {
            return None;
        }

        let p = ray.at(t);
        let planar_hitpt_vector = p - self.q;
        let u = self.w.dot(&planar_hitpt_vector.cross(&self.v_hat));
        let v = 1.0 - self.w.dot(&self.u_hat.cross(&planar_hitpt_vector));

        if !(Interval::UNIT.contains(u) && Interval::UNIT.contains(v)) {
            return None;
        }
        return Some(Intersection::new(
            ray.at(t),
            t,
            self.normal,
            self.mat.as_ref(),
            ray,
            Vector2::new(u, v),
        ));
    }
}
