use crate::geom::{intersectable::Intersection, ray::Ray};

use super::color::Color;

pub trait Material {
    fn scatter<'o>(&'o self, intersection : &Intersection<'o,'_>) -> Option<(Color,Ray)>;
}