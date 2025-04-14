use crate::{
    geom::{intersection::Intersection, ray::Ray},
    util::{random_unit_vec3, reflect},
};

use super::{
    color::Color,
    material::{Material, Scatter},
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, inter: &Intersection) -> Option<Scatter> {
        let refl = reflect(&inter.ray_in().dir(), &inter.normal());
        let scattered = refl + random_unit_vec3().scale(self.fuzz);
        if scattered.dot(&inter.normal()) > 0.0 {
            Some(Scatter::new(
                self.albedo,
                Ray::new_normalize(inter.point(), scattered),
            ))
        } else {
            None
        }
    }
}
