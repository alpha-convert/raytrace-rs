use std::iter::Map;

use nalgebra::{UnitVector3, Vector, Vector3};
use serde::Deserialize;

use crate::lighting::color::Color;

#[derive(Deserialize, Debug)]
pub struct SceneDesc {
    bsdfs : Vec<BDSFDesc>
}


#[derive(Deserialize, Debug)]
pub struct BDSFDesc {
    name : String,
    albedo : AlbedoDesc,
    r#type : String
}

#[derive(Deserialize, Debug)]
enum AlbedoDesc {
    Scalar(f64),
    Vector([f64;3])
}