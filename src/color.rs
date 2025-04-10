use std::ops::Add;

use nalgebra::Vector3;

pub struct Color {
    v : Vector3<f64>
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

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {v : self.v.add(rhs.v)}
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB((self.v.x * 256.0).floor() as u8, (self.v.y * 256.0).floor() as u8, (self.v.z * 256.0).floor() as u8)
    }
}