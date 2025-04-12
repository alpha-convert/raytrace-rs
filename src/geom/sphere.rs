use nalgebra::{Unit, Vector3};

use crate::{
    geom::{
        intersectable::{Intersectable, Intersection},
        ray::Ray,
    },
    lighting::material::Material,
};

use super::{aabb::AABB, interval::Interval};

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Box<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Box<dyn Material>) -> Self {
        let rad3 = Vector3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - rad3, center + rad3);
        Sphere {
            center: center,
            radius: radius,
            material: material,
            bbox,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect<'o>(&'o self, ray: Ray, i: Interval) -> Option<Intersection<'o>> {
        if !self.bbox.intersect(ray, i) {
            return None;
        }

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

        Some(Intersection::new(point, dist, normal, &self.material, ray))
    }
}
