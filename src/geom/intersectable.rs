use std::sync::Arc;

use nalgebra::{Unit, Vector2, Vector3};

use crate::geom::ray::Ray;
use crate::lighting::material::Material;

use super::interval::Interval;

pub struct Intersection<'o> {
    point: Vector3<f64>,
    dist: f64,
    normal: Unit<Vector3<f64>>,
    material: &'o Box<dyn Material>,
    ray_in: Ray,
    uv: Vector2<f64>,
}

impl<'o> Intersection<'o> {
    pub fn new(
        point: Vector3<f64>,
        dist: f64,
        normal: Unit<Vector3<f64>>,
        material: &'o Box<dyn Material>,
        ray_in: Ray,
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

    pub fn material(&self) -> &'o Box<dyn Material> {
        self.material
    }

    pub fn ray_in(&self) -> Ray {
        self.ray_in
    }
}

pub trait Intersectable: Send + Sync {
    // It might be more efficient to pass in a &mut Option<Intersectoin>, but that's ugly.
    fn intersect<'o>(&'o self, ray: Ray, i: Interval) -> Option<Intersection<'o>>;
}

// impl Intersectable for Arc<dyn Intersectable> {
//     fn intersect<'o>(&'o self, ray : Ray, i : Interval) -> Option<Intersection<'o>> {
//         (**self).intersect(ray,i)
//     }
// }
