use std::{sync::Arc};

use nalgebra::Vector3;

use crate::math::{interval::Interval, ray::Ray};

use super::{intersection::Intersection, Geom};

pub struct Scaling {
    scale_factor_x : f64,
    scale_factor_y : f64,
    scale_factor_z : f64,
    geom : Arc<dyn Geom>
}

impl Scaling {
    pub fn new(scale: Vector3<f64>, geom : Arc<dyn Geom>) -> Self {
        assert!(0.0 < scale.x);;
        assert!(0.0 < scale.y);;
        assert!(0.0 < scale.z);;
        Scaling {
            scale_factor_x : 1.0 / scale.x,
            scale_factor_y : 1.0 / scale.y,
            scale_factor_z : 1.0 / scale.z,
            geom: geom,
        }
    }
}

impl Geom for Scaling {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        todo!()
    }

    fn bbox(&self) -> super::aabb::AABB {
        todo!()
    }
}
