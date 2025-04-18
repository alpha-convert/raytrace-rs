use std::sync::Arc;

use nalgebra::{Matrix3, Rotation3, Unit, Vector3};

use crate::math::{interval::Interval, ray::Ray};

use super::{
    Geomable, aabb::AABB, bbox::Bbox, intersectable::Intersectable, intersection::Intersection,
};

pub struct Rotation<T> {
    rotation: Rotation3<f64>,
    rotation_inv: Rotation3<f64>,
    inner: T,
}

impl<T> Rotation<T> {
    pub fn from_axis_angle(axis: Unit<Vector3<f64>>, angle_rad: f64, inner: T) -> Self {
        let rotation = Rotation3::from_axis_angle(&axis, angle_rad);
        let rotation_inv = rotation.inverse();

        Rotation {
            rotation,
            rotation_inv,
            inner,
        }
    }

    // Alternative constructor using Euler angles (roll, pitch, yaw)
    pub fn from_euler(roll: f64, pitch: f64, yaw: f64, inner: T) -> Self {
        let rotation = Rotation3::from_euler_angles(roll, pitch, yaw);
        let rotation_inv = rotation.inverse();

        Rotation {
            rotation,
            rotation_inv,
            inner,
        }
    }
}

impl<T: Intersectable> Intersectable for Rotation<T> {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        // Transform ray to object space
        let rotated_origin = self.rotation_inv * ray.origin();
        let rotated_dir = self.rotation_inv * ray.dir().into_inner();

        let rotated_dir = Unit::new_normalize(rotated_dir);
        let rotated_ray = Ray::new(rotated_origin, rotated_dir);

        // Intersect with the underlying geometry
        let int = self.inner.intersect(rotated_ray, i)?;

        // Transform intersection point and normal back to world space
        let point = self.rotation * int.point();

        // Calculate actual distance in world space
        let dist = (point - ray.origin()).norm();

        // Transform normal back to world space
        let normal = Unit::new_normalize(self.rotation * int.normal().into_inner());

        Some(Intersection::new(
            point,
            dist,
            normal,
            int.material(),
            int.uv(),
        ))
    }
}

impl<T: Geomable> Geomable for Rotation<T> {
    fn into_geoms(self) -> impl Iterator<Item = super::Geom> {
        //NOTE: this makes stupid numbers of copies of the rotation matrices
        self.inner.into_geoms().map(move |g| {
            super::Geom::Rot(Box::new(Rotation {
                rotation: self.rotation,
                rotation_inv: self.rotation_inv,
                inner: g,
            }))
        })
    }
}

impl<T: Bbox> Bbox for Rotation<T> {
    fn bbox(&self) -> AABB {
        let inner_bbox = self.inner.bbox();

        // Get the corners of the inner bounding box
        let min = inner_bbox.min();
        let max = inner_bbox.max();

        // Create all 8 corners of the box
        let corners = [
            Vector3::new(min.x, min.y, min.z),
            Vector3::new(max.x, min.y, min.z),
            Vector3::new(min.x, max.y, min.z),
            Vector3::new(max.x, max.y, min.z),
            Vector3::new(min.x, min.y, max.z),
            Vector3::new(max.x, min.y, max.z),
            Vector3::new(min.x, max.y, max.z),
            Vector3::new(max.x, max.y, max.z),
        ];

        // Rotate all corners and find the new bounding box
        let rotated_corners: Vec<Vector3<f64>> = corners
            .iter()
            .map(|corner| self.rotation * corner)
            .collect();

        // Find the min and max of the rotated corners
        let mut new_min = rotated_corners[0];
        let mut new_max = rotated_corners[0];

        for corner in &rotated_corners[1..] {
            new_min.x = new_min.x.min(corner.x);
            new_min.y = new_min.y.min(corner.y);
            new_min.z = new_min.z.min(corner.z);

            new_max.x = new_max.x.max(corner.x);
            new_max.y = new_max.y.max(corner.y);
            new_max.z = new_max.z.max(corner.z);
        }

        AABB::from_points(new_min, new_max)
    }
}
