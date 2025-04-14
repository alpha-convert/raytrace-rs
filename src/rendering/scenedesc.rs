use std::collections::HashMap;

use nalgebra::Vector3;

use crate::lighting::color::Color;

pub enum TextureDesc {
    Solid {
        albedo: Color,
    },
    Checkerboard {
        tex1: String,
        tex2: String,
        checker_size: f64,
    },
    Image {
        fname: String,
    },
    ScaleTex {
        scale_u: f64,
        scale_v: f64,
        tex: String,
    },
}
pub enum MaterialDesc {
    DiffuseLight { tex: String },
    Lambertian { tex: String },
    Metal { albedo: Color, fuzz: f64 },
}

pub enum GeomDesc {
    Cube {
        c: Vector3<f64>,
        r: f64,
        mat: String,
    },
    Plane {
        center: Vector3<f64>,
        normal: Vector3<f64>,
        u_hat: Vector3<f64>,
        v_hat: Vector3<f64>,
        mat: String,
    },
    Quad {
        q: Vector3<f64>,
        u: Vector3<f64>,
        v: Vector3<f64>,
        mat: String,
    },
    Sphere {
        c: Vector3<f64>,
        r: f64,
        mat: String,
    },
    Translation {
        by: Vector3<f64>,
        gd: Box<GeomDesc>,
    },
}

pub struct SceneDesc {
    pub textures: HashMap<String, TextureDesc>,
    pub materials: HashMap<String, MaterialDesc>,
    pub geoms: Vec<GeomDesc>,
    pub background_color: Color,
}

fn parse_texture(typ: String, obj: &serde_json::Value) -> TextureDesc {
    if typ == "solid" {
        let albedo = obj
            .get("albedo")
            .expect("Solid texture has albedo")
            .as_array()
            .expect("Albedo is vec3");
        let r = albedo[0].as_f64().expect("R is f64");
        let g = albedo[1].as_f64().expect("g is f64");
        let b = albedo[2].as_f64().expect("B is f64");
        return TextureDesc::Solid {
            albedo: Color::new(r, g, b),
        };
    } else {
        unimplemented!()
    }
}

fn parse_textures(v: &Vec<serde_json::Value>) -> HashMap<String, TextureDesc> {
    let mut m = HashMap::new();
    for obj in v {
        let typ = obj
            .get("type")
            .expect("Texture has a type")
            .as_str()
            .expect("Type is a string");
        let name = obj
            .get("name")
            .expect("Texture has a name")
            .as_str()
            .expect("Name is a string");
        let tex = parse_texture(String::from(typ), obj);
        m.insert(String::from(name), tex);
    }

    m
}
fn parse_material(typ: String, obj: &serde_json::Value) -> MaterialDesc {
    todo!()
}

fn parse_materials(v: &Vec<serde_json::Value>) -> HashMap<String, MaterialDesc> {
    let mut m = HashMap::new();
    for obj in v {
        let typ = obj
            .get("type")
            .expect("Material has a type")
            .as_str()
            .expect("Material type is a string");
        let name = obj
            .get("name")
            .expect("Material has a name")
            .as_str()
            .expect("Material name is a string");
        let tex = parse_material(String::from(typ), obj);
        m.insert(String::from(name), tex);
    }

    m
}

impl From<serde_json::Value> for SceneDesc {
    fn from(value: serde_json::Value) -> Self {
        let textures = value
            .get("textures")
            .expect(" Scene has textures")
            .as_array()
            .expect("Textures is an array");
        let textures = parse_textures(textures);

        let materials = value
            .get("materials")
            .expect("Scene has materials")
            .as_array()
            .expect(" Materials is an array");
        let materials = parse_materials(materials);

        let geoms = todo!();
        let background_color = todo!();

        SceneDesc {
            textures,
            materials,
            geoms,
            background_color,
        }
    }
}
