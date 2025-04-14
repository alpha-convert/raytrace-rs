use std::{fs::File, io::BufReader, sync::Arc};

use nalgebra::{Unit, Vector3};
use obj::{IndexTuple, Obj, SimplePolygon};

use crate::{
    lighting::material::Material,
    math::{interval::Interval, ray::Ray},
};

use super::{Geom, bvh::BVH, intersection::Intersection, triangle::Triangle};

pub struct TriMesh {
    faces: BVH<Triangle>,
}

impl TriMesh {
    pub fn from_fname(fname: &str, mat: Arc<dyn Material>) -> Self {
        let obj = obj::Obj::load(fname).unwrap();

        let verts = obj.data.position;
        let faces = &obj.data.objects[0].groups[0].polys;

        let mut tris: Vec<Triangle> = Vec::with_capacity(faces.len());
        for SimplePolygon(face) in faces {
            let IndexTuple(vidx0, _, _) = face[0];
            let IndexTuple(vidx1, _, _) = face[1];
            let IndexTuple(vidx2, _, _) = face[2];
            let a: Vector3<f64> = Vector3::from_column_slice(verts.get(vidx0).unwrap()).cast();
            let b: Vector3<f64> = Vector3::from_column_slice(verts.get(vidx1).unwrap()).cast();
            let c: Vector3<f64> = Vector3::from_column_slice(verts.get(vidx2).unwrap()).cast();

            let edge1 = b - a;
            let edge2 = c - a;

            let normal = Unit::new_normalize(edge1.cross(&edge2));

            tris.push(Triangle::new(a, b, c, normal, mat.clone()));
        }

        TriMesh {
            faces: BVH::construct(tris),
        }
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
