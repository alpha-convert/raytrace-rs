use std::sync::Arc;

use nalgebra::Vector3;

use crate::{
    lighting::material::Material,
    math::{interval::Interval, ray::Ray},
};

use super::{aabb::AABB, intersectable::{self, Intersectable}, intersection::Intersection, quad::Quad};

pub struct Cube {
    faces: Arc<[Box<Quad>; 6]>,
}

impl Cube {
    pub fn new(c: Vector3<f64>, r: f64, mat: Arc<dyn Material>) -> Self {
        // c is the center of the cube, r is the half-width (radius)

        // Note: For each face, we need to order the vertices correctly so that
        // the cross product of u_hat and v_hat points outward

        // Front face (positive Z)
        let front = Quad::new(
            Vector3::new(c.x - r, c.y - r, c.z + r), // bottom-left front
            Vector3::new(2.0 * r, 0.0, 0.0),         // u vector along positive x
            Vector3::new(0.0, 2.0 * r, 0.0),         // v vector along positive y
            mat.clone(),
        );

        // Back face (negative Z)
        let back = Quad::new(
            Vector3::new(c.x - r, c.y - r, c.z - r), // bottom-left back
            Vector3::new(0.0, 2.0 * r, 0.0),         // u vector along positive y
            Vector3::new(2.0 * r, 0.0, 0.0),         // v vector along positive x
            mat.clone(),
        );

        // Right face (positive X)
        let right = Quad::new(
            Vector3::new(c.x + r, c.y - r, c.z - r), // bottom-right back
            Vector3::new(0.0, 2.0 * r, 0.0),         // u vector along positive y
            Vector3::new(0.0, 0.0, 2.0 * r),         // v vector along positive z
            mat.clone(),
        );

        // Left face (negative X)
        let left = Quad::new(
            Vector3::new(c.x - r, c.y - r, c.z - r), // bottom-left back
            Vector3::new(0.0, 0.0, 2.0 * r),         // u vector along positive z
            Vector3::new(0.0, 2.0 * r, 0.0),         // v vector along positive y
            mat.clone(),
        );

        // Top face (positive Y)
        let top = Quad::new(
            Vector3::new(c.x - r, c.y + r, c.z - r), // top-left back
            Vector3::new(2.0 * r, 0.0, 0.0),         // u vector along positive x
            Vector3::new(0.0, 0.0, 2.0 * r),         // v vector along positive z
            mat.clone(),
        );

        // Bottom face (negative Y)
        let bottom = Quad::new(
            Vector3::new(c.x - r, c.y - r, c.z - r), // bottom-left back
            Vector3::new(0.0, 0.0, 2.0 * r),         // u vector along positive z
            Vector3::new(2.0 * r, 0.0, 0.0),         // v vector along positive x
            mat.clone(),
        );

        Cube {
            faces: [
                Box::new(front),
                Box::new(back),
                Box::new(top),
                Box::new(bottom),
                Box::new(right),
                Box::new(left),
            ]
            .into(),
        }
    }
}

impl Intersectable for Cube {
    fn intersect<'r>(&'r self, ray: Ray, i: Interval) -> Option<Intersection<'r>> {
        (self.faces)
            .as_ref()
            .into_iter()
            .filter_map(|face| face.intersect(ray, i))
            .min_by(Intersection::dist_compare)
    }

    // fn bbox(&self) -> AABB {
    //     AABB::union_all(self.faces.as_ref().into_iter().map(|obj| obj.bbox()))
    // }
}
