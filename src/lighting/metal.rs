
use crate::{geom::{intersectable::Intersection, ray::Ray}, util::{random_unit_vec3, reflect}};

use super::{color::Color, material::Material};

#[derive(Debug,Clone,Copy)]
pub struct Metal {
    albedo : Color,
    fuzz : f64
}

impl Metal {
    pub fn new(albedo : Color, fuzz:f64) -> Self {
        Metal {albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter<'o,'r>(&'o self, inter : &Intersection<'o>) -> Option<(Color,Ray)> {
        let refl = reflect(&inter.ray_in().dir(), &inter.normal());
        let scattered = refl + random_unit_vec3().scale(self.fuzz);
        if scattered.dot(&inter.normal()) > 0.0 {
            Some((self.albedo,Ray::new_normalize(inter.point(), scattered)))
        } else {
            None
        }
    }
}