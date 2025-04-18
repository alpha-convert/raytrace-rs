use std::ops::Deref;
use std::sync::Arc;

use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::raypacket::RayPacket;
use aabb::AABB;
use bbox::Bbox;
use intersectable::Intersectable;
use intersection::Intersection;
use quad::Quad;
use rotation::Rotation;
use scaling::Scaling;
use sphere::Sphere;
use translation::Translation;
use triangle::Triangle;

pub mod aabb;
pub mod bvh;
pub mod cube;
pub mod intersectable;
pub mod intersection;
pub mod quad;
pub mod rotation;
pub mod scaling;
pub mod sphere;
pub mod translation;
pub mod triangle;
pub mod trimesh;
pub mod bvhcache;
pub mod bbox;


pub enum Geom {
    Quad(Box<Quad>),
    Tri(Box<Triangle>),
    Sphere(Box<Sphere>),
    Rot(Box<Rotation<Geom>>),
    Scale(Box<Scaling<Geom>>),
    Trans(Box<Translation<Geom>>)
}

impl Intersectable for Geom {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        match self {
            Geom::Quad(q) => q.intersect(ray, i),
            Geom::Tri(triangle) => triangle.intersect(ray, i),
            Geom::Sphere(sphere) => sphere.intersect(ray, i),
            Geom::Rot(rotation) => rotation.intersect(ray, i),
            Geom::Scale(scaling) => scaling.intersect(ray, i),
            Geom::Trans(translation) => translation.intersect(ray, i),
        }
    }
}

impl Bbox for Geom {
    fn bbox(&self) -> AABB {
        match self {
            Geom::Quad(quad) => quad.bbox(),
            Geom::Tri(triangle) => triangle.bbox(),
            Geom::Sphere(sphere) => sphere.bbox(),
            Geom::Rot(rotation) => rotation.bbox(),
            Geom::Scale(scaling) => scaling.bbox(),
            Geom::Trans(translation) => translation.bbox(),
        }
    }
}

pub trait Geomable {
    fn into_geoms(self) -> impl Iterator<Item = Geom>;
}

impl Geomable for Geom {
    fn into_geoms(self) -> impl Iterator<Item = Geom> {
        std::iter::once(self)
    }
}

impl<T,I> Geomable for I
    where T : Geomable,
          I : IntoIterator<Item = T>
{
    fn into_geoms(self) -> impl Iterator<Item = Geom> {
        self.into_iter().flat_map(|t| {t.into_geoms()})
    }
}