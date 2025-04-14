use std::{f64::consts::PI, sync::Arc};

use nalgebra::{Unit, UnitVector3, Vector2, Vector3};

use crate::{
    geom::{Geom, intersection::Intersection, ray::Ray},
    lighting::material::Material,
};

use super::{aabb::AABB, interval::Interval};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Arc<dyn Material>) -> Self {
        let rad3 = Vector3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - rad3, center + rad3);
        Sphere {
            center: center,
            radius: radius,
            material: material,
            bbox,
        }
    }

    /// Ripped from raytracing the next week. Given a point on the unit sphere,
    /// return the uv coordinates for that point.
    fn unit_sphere_uv(v: &UnitVector3<f64>) -> Vector2<f64> {
        let theta = f64::acos(-v.y);
        let phi = f64::atan2(-v.z, v.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        Vector2::new(u, v)
    }
}

impl Geom for Sphere {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        let oc = self.center - ray.origin();

        let h = ray.dir().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - c;

        if discriminant < 0.0 {
            return None;
        }

        let mut dist = h - discriminant.sqrt();

        if !i.contains(dist) {
            dist = h + discriminant.sqrt();
            if !i.contains(dist) {
                return None;
            }
        }

        let point = ray.at(dist);
        let normal = Unit::new_normalize(point - self.center); // can also divide by the radius...

        let uv = Sphere::unit_sphere_uv(&normal);

        Some(Intersection::new(
            point,
            dist,
            normal,
            self.material.as_ref(),
            ray,
            uv,
        ))
    }

    fn bbox(&self) -> AABB {
        self.bbox.clone()
    }
}
