use std::cmp::Ordering;
use std::f64::NAN;

use nalgebra::{Unit, UnitVector3, Vector2, Vector3};

use crate::lighting::material::Material;
use crate::math::ray::Ray;

pub struct Intersection<'r> {
    point: Vector3<f64>,
    dist: f64, //NOTE: distances are relative, not absolute (because of scaling)
    normal: Unit<Vector3<f64>>,
    material: &'r dyn Material,
    ray_in_dir: UnitVector3<f64>,
    uv: Vector2<f64>,
}

impl<'r> Intersection<'r> {
    pub fn new(
        point: Vector3<f64>,
        dist: f64,
        normal: Unit<Vector3<f64>>,
        material: &'r dyn Material,
        ray_in_dir: UnitVector3<f64>,
        uv: Vector2<f64>,
    ) -> Self {
        Intersection {
            point: point,
            dist: dist,
            normal: normal,
            material: material,
            ray_in_dir,
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

    pub fn ray_in_dir(&self) -> UnitVector3<f64> {
        self.ray_in_dir
    }
}
