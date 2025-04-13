use crate::geom::{intersectable::Intersection, ray::Ray};

use super::color::Color;

pub trait Material: Sync + Send {
    fn scatter(&self, inter: &Intersection) -> Option<(Color, Ray)>;
}
