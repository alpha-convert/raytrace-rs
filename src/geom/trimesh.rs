use std::sync::Arc;

use nalgebra::{Unit, Vector3};

use crate::{lighting::material::Material, math::{interval::Interval, ray::Ray}};

use super::{bvh::BVH, intersection::Intersection, triangle::Triangle, Geom};

pub struct TriMesh {
    faces : BVH<Triangle>
}

impl TriMesh {
    pub fn from_verts_faces(verts : &[Vector3<f64>], faces : &[Vector3<usize>], mat : Arc<dyn Material>) -> Self {
        let mut tris : Vec<Triangle> = Vec::with_capacity(faces.len());
        for face in faces {
            let vidx0 = face[0];
            let vidx1 = face[1];
            let vidx2 = face[2];
            let a= verts.get(vidx0 - 1).unwrap();
            let b= verts.get(vidx1 - 1).unwrap();
            let c= verts.get(vidx2 - 1).unwrap();

            let edge1 = b - a;
            let edge2 = c - a;

            let normal = Unit::new_normalize(edge1.cross(&edge2));

            tris.push(Triangle::new(*a,*b,*c,normal,mat.clone()));
        };

        TriMesh { faces: BVH::construct(tris) }
    }

}

impl Geom for TriMesh {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        self.faces.intersect(ray, i)
    }

    fn bbox(&self) -> super::aabb::AABB {
        self.faces.bbox()
    }
}