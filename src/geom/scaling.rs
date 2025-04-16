use std::sync::Arc;

use nalgebra::{SimdPartialOrd, Unit, Vector3};

use crate::math::{interval::Interval, ray::Ray};

use super::{Geom, aabb::AABB, intersection::Intersection};

pub struct Scaling {
    scale: Vector3<f64>,
    scale_inv: Vector3<f64>,
    geom: Arc<dyn Geom>,
}

impl Scaling {
    pub fn new(scale: Vector3<f64>, geom: Arc<dyn Geom>) -> Self {
        assert!(0.0 < scale.x);
        assert!(0.0 < scale.y);
        assert!(0.0 < scale.z);
        Scaling {
            scale,
            scale_inv: Vector3::new(1.0 / scale.x, 1.0 / scale.y, 1.0 / scale.z),
            geom: geom,
        }
    }
}

impl Geom for Scaling {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        let scaled_origin = ray.origin().component_mul(&self.scale_inv);
        let scaled_dir = ray.dir().component_mul(&self.scale_inv);
        if scaled_dir.magnitude_squared() < 1e-10 {
            return None;
        }
        let scaled_dir = Unit::new_normalize(scaled_dir);

        let scaled_ray = Ray::new(scaled_origin, scaled_dir);

        let int = self.geom.intersect(scaled_ray, i)?;

        let point = int.point().component_mul(&self.scale);
        let dist = (point - ray.origin()).norm(); //recompute the distance using the point.
        let scaled_normal = int.normal().component_mul(&self.scale_inv);
        if scaled_normal.magnitude_squared() < 1e-10 {
            return None;
        }
        let scaled_normal = Unit::new_normalize(scaled_normal);

        Some(Intersection::new(
            point,
            dist,
            scaled_normal,
            int.material(),
            int.uv(),
        ))
    }

    fn bbox(&self) -> super::aabb::AABB {
        let inner_bbox = self.geom.bbox();
        let min = inner_bbox.min();
        let max = inner_bbox.max();

        let scaled_min = min.component_mul(&self.scale);
        let scaled_max = max.component_mul(&self.scale);

        // Ensure min < max for each dimension
        let new_min = scaled_min.simd_min(scaled_max);
        let new_max = scaled_min.simd_max(scaled_max);

        AABB::from_points(new_min, new_max)
    }
}
