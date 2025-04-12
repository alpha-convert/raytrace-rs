use crate::geom::{intersectable::Intersection, ray::Ray};

use super::color::Color;

pub trait Material: Sync + Send {
    fn scatter<'o>(&'o self, inter: &Intersection<'o>) -> Option<(Color, Ray)>;
}
