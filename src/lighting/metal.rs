
use crate::{geom::{intersectable::Intersection, ray::Ray}, util::{reflect}};

use super::{color::Color, material::Material};

#[derive(Debug,Clone,Copy)]
pub struct Metal {
    albedo : Color
}

impl Metal {
    pub fn new(albedo : Color) -> Self {
        Metal {albedo}
    }
}

impl Material for Metal {
    fn scatter<'o,'r>(&'o self, inter : &Intersection<'o,'r>) -> Option<(Color,Ray)> {
        let dir = reflect(inter.ray_in().dir(), &inter.normal());
        Some((self.albedo,Ray::new_normalize(inter.point(), dir)))
    }
}