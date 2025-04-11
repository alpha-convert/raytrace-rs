use std::{collections::VecDeque, ops::{Add, Mul}};

use nalgebra::{UnitVector3, Vector3};

#[derive(Debug,Clone,Copy)]
pub struct Color {
    v : Vector3<f64>
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl Color {
    pub fn new(r : f64, g : f64, b : f64) -> Self {
        assert!(0.0 <= r && r <= 1.0);
        assert!(0.0 <= g && g <= 1.0);
        assert!(0.0 <= b && b <= 1.0);

        Color { v: Vector3::new(r, g, b) }
    }

    pub fn scale(&self, f : f64) -> Self {
        Color { v: self.v.scale(f) }
    }

    pub(crate) fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
    
    pub(crate) fn black() -> Self {
        Color::new(0.0, 0.0,0.0)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {v : self.v.add(rhs.v)}
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color { v : Vector3::new(self.v.x * rhs.v.x, self.v.y * rhs.v.y, self.v.z*rhs.v.z)}
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        // Apply gamma correction (as floats) at this point, before mapping into u8 space.
        sdl2::pixels::Color::RGB((self.v.x.sqrt() * 256.0).floor() as u8, (self.v.y.sqrt() * 256.0).floor() as u8, (self.v.z.sqrt() * 256.0).floor() as u8)
    }
}

impl From<&UnitVector3<f64>> for Color {
    fn from(value: &UnitVector3<f64>) -> Self {
        let r = (1.0 + value.x)/2.0;
        let g = (1.0 + value.y)/2.0;
        let b = (1.0 + value.z)/2.0;
        Self::new(r,g,b)
    }
}