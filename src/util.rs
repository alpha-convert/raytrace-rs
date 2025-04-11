use nalgebra::{Unit, UnitVector3, Vector3};
use rand::Rng;

pub fn random_vec3(lo : f64, hi:f64) -> Vector3<f64> {
    let mut rng = rand::rng();
    let x : f64 = (hi - lo) * rng.random::<f64>() - (hi - lo)/2.0;
    let y : f64 = (hi - lo) * rng.random::<f64>() - (hi - lo)/2.0;
    let z : f64 = (hi - lo) * rng.random::<f64>() - (hi - lo)/2.0;
    Vector3::new(x, y, z)
}

pub fn random_unit_vec3() -> UnitVector3<f64> {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.norm_squared() <= 1.0 {
            return Unit::new_normalize(p)
        }
    }
}

pub fn random_on_hemisphere(normal : &UnitVector3<f64>) -> UnitVector3<f64> {
    let rand = random_unit_vec3();
    if normal.dot(&rand) > 0.0 {
        rand
    } else {
        -rand
    }
}