use std::sync::Arc;

use nalgebra::{UnitVector3, Vector2, Vector3};

use crate::{
    lighting::material::Material,
    math::{interval::Interval, ray::Ray},
};

use super::{Geom, aabb::AABB, intersection::Intersection};

#[derive(Clone)]
pub struct Triangle {
    a: Vector3<f64>,
    b: Vector3<f64>,
    c: Vector3<f64>,
    normal: UnitVector3<f64>,

    bbox: AABB,

    mat: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(
        a: Vector3<f64>,
        b: Vector3<f64>,
        c: Vector3<f64>,
        n: UnitVector3<f64>,
        mat: Arc<dyn Material>,
    ) -> Self {
        let bb1 = AABB::from_points(a, b);
        let bb2 = AABB::from_points(a, c);
        let bbox = AABB::union(&bb1, &bb2);
        Triangle {
            a: a,
            b: b,
            c: c,
            normal: n,
            bbox: bbox,
            mat,
        }
    }
}

impl Geom for Triangle {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        //Stolen from the wiki page on Möller–Trumbore.

        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = ray.dir().cross(&e2);
        let det = e1.dot(&ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - self.a;
        let u = inv_det * s.dot(&ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(&e1);
        let v = inv_det * ray.dir().dot(&s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inv_det * e2.dot(&s_cross_e1);

        if !i.contains(t) {
            return None;
        }

        let pt = ray.at(t);
        let uv = Vector2::new(u, v);
        Some(Intersection::new(
            pt,
            t,
            self.normal,
            self.mat.as_ref(),
            ray,
            uv,
        ))
    }

    fn bbox(&self) -> super::aabb::AABB {
        self.bbox.clone()
    }
}
