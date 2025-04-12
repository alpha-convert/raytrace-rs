use nalgebra::{Unit, UnitVector3, Vector3};
use rand::Rng;

pub fn random_vec3(lo: f64, hi: f64) -> Vector3<f64> {
    let mut rng = rand::rng();
    let x: f64 = (hi - lo) * rng.random::<f64>() - (hi - lo) / 2.0;
    let y: f64 = (hi - lo) * rng.random::<f64>() - (hi - lo) / 2.0;
    let z: f64 = (hi - lo) * rng.random::<f64>() - (hi - lo) / 2.0;
    Vector3::new(x, y, z)
}

pub fn random_unit_vec3() -> UnitVector3<f64> {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.norm_squared() <= 1.0 {
            return Unit::new_normalize(p);
        }
    }
}

pub fn random_on_hemisphere(normal: &UnitVector3<f64>) -> UnitVector3<f64> {
    let rand = random_unit_vec3();
    if normal.dot(&rand) > 0.0 { rand } else { -rand }
}

const EPS: f64 = 1e-8;

pub fn is_small(v: Vector3<f64>) -> bool {
    v.x.abs() < EPS && v.y.abs() < EPS && v.z.abs() < EPS
}

pub fn reflect(v: &Vector3<f64>, about: &UnitVector3<f64>) -> Vector3<f64> {
    return *v - about.scale(2.0 * v.dot(about));
}
