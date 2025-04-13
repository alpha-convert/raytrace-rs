
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SceneDesc {
    bsdfs: Vec<BDSFDesc>,
}

#[derive(Deserialize, Debug)]
pub struct BDSFDesc {
    name: String,
    albedo: AlbedoDesc,
    r#type: String,
}

#[derive(Deserialize, Debug)]
enum AlbedoDesc {
    Scalar(f64),
    Vector([f64; 3]),
}
