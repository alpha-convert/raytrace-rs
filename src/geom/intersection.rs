use std::cmp::Ordering;
use std::f64::NAN;

use nalgebra::{Unit, UnitVector3, Vector2, Vector3};

use crate::lighting::material::Material;

pub struct Intersection<'r> {
    point: Vector3<f64>,
    dist: f64, //note: this can always be recomputed from the point and the ray at top level.
    normal: Unit<Vector3<f64>>,
    material: &'r dyn Material,
    uv: Vector2<f64>,
}

impl<'r> Intersection<'r> {
    pub fn new(
        point: Vector3<f64>,
        dist: f64,
        normal: Unit<Vector3<f64>>,
        material: &'r dyn Material,
        uv: Vector2<f64>,
    ) -> Self {
        Intersection {
            point: point,
            dist: dist,
            normal: normal,
            material: material,
            uv,
        }
    }

    pub fn dist_compare(&self, other: &Self) -> Ordering {
        assert!(!self.dist.is_nan());
        assert!(!other.dist.is_nan());

        if self.dist < other.dist {
            return Ordering::Less;
        } else if self.dist > other.dist {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
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
}
