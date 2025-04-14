use std::cmp::Ordering;
use std::f64::NAN;
use std::sync::Arc;

use nalgebra::{Unit, Vector2, Vector3};

use crate::geom::ray::Ray;
use crate::lighting::material::Material;

use super::interval::Interval;

pub struct Intersection<'r> {
    point: Vector3<f64>,
    dist: f64,
    normal: Unit<Vector3<f64>>,
    material: &'r dyn Material,
    ray_in: &'r Ray,
    uv: Vector2<f64>,
}

impl<'r> Intersection<'r> {
    pub fn new(
        point: Vector3<f64>,
        dist: f64,
        normal: Unit<Vector3<f64>>,
        material: &'r dyn Material,
        ray_in: &'r Ray,
        uv: Vector2<f64>,
    ) -> Self {
        Intersection {
            point: point,
            dist: dist,
            normal: normal,
            material: material,
            ray_in,
            uv,
        }
    }

    pub fn dist_compare(&self, other: &Self) -> Ordering {
        assert!(self.dist != NAN);
        assert!(other.dist != NAN);

        if self.dist < other.dist {
            return Ordering::Less;
        } else if self.dist > other.dist {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }

    pub fn dist(&self) -> f64 {
        self.dist
    }

    pub fn normal(&self) -> Unit<Vector3<f64>> {
        self.normal
    }

    pub fn point(&self) -> Vector3<f64> {
        self.point
    }

    pub fn uv(&self) -> Vector2<f64> {
        self.uv
    }

    pub fn point_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.point
    }

    pub fn material(&self) -> &'r dyn Material {
        self.material
    }

    pub fn ray_in(&self) -> &'r Ray {
        self.ray_in
    }
}

pub trait Intersectable: Send + Sync {
    // It might be more efficient to pass in a &mut Option<Intersectoin>, but that's ugly.
    fn intersect<'r>(&'r self, ray: &'r Ray, i: Interval) -> Option<Intersection<'r>>;
}

// impl Intersectable for Arc<dyn Intersectable> {
//     fn intersect<'o>(&'o self, ray : Ray, i : Interval) -> Option<Intersection<'o>> {
//         (**self).intersect(ray,i)
//     }
// }
