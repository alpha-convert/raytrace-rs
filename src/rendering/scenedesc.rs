use std::collections::HashMap;

use nalgebra::Vector3;
use serde_json::Value;

use crate::{lighting::color::Color, math::interval::Interval};

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
    // Plane {
    //     center: Vector3<f64>,
    //     normal: Vector3<f64>,
    //     u_hat: Vector3<f64>,
    //     v_hat: Vector3<f64>,
    //     mat: String,
    // },
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

fn parse_color(obj: &serde_json::Value) -> Color {
    let obj = obj.as_array().expect("Color is array");
    assert!(obj.len() == 3);
    let r = obj[0].as_f64().expect("R is f64");
    let g = obj[1].as_f64().expect("g is f64");
    let b = obj[2].as_f64().expect("B is f64");
    assert!(Interval::UNIT.contains(r));
    assert!(Interval::UNIT.contains(g));
    assert!(Interval::UNIT.contains(b));
    Color::new(r, g, b)
}

fn parse_vec3(obj: &serde_json::Value) -> Vector3<f64> {
    let obj = obj.as_array().expect("Color is array");
    assert!(obj.len() == 3);
    let x = obj[0].as_f64().expect("X is f64");
    let y = obj[1].as_f64().expect("Y is f64");
    let z = obj[2].as_f64().expect("Z is f64");
    Vector3::new(x, y, z)
}

fn parse_texture(typ: String, obj: &serde_json::Value) -> TextureDesc {
    if typ == "solid" {
        let albedo = obj.get("albedo").expect("Solid texture has albedo");
        return TextureDesc::Solid {
            albedo: parse_color(&albedo),
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
    if typ == "diffuselight" {
        let tex = obj
            .get("tex")
            .expect("DiffuseLight Material has a tex")
            .as_str()
            .expect("tex is a string");
        MaterialDesc::DiffuseLight {
            tex: String::from(tex),
        }
    } else if typ == "lambert" {
        let tex = obj
            .get("tex")
            .expect("Lambert Material has a tex")
            .as_str()
            .expect("tex is a string");
        MaterialDesc::DiffuseLight {
            tex: String::from(tex),
        }
    } else if typ == "metal" {
        let fuzz = obj
            .get("fuzz")
            .expect("Metal Material has a fuzz")
            .as_f64()
            .expect("fuzz is a number");
        let albedo = obj.get("albedo").expect("Metal texture has albedo");
        MaterialDesc::Metal {
            albedo: parse_color(albedo),
            fuzz: fuzz,
        }
    } else {
        unimplemented!()
    }
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

fn parse_geom(geom: &Value) -> GeomDesc {
    let typ = geom
        .get("type")
        .expect("Geom has a type")
        .as_str()
        .expect("type is a string");
    if typ == "quad" {
        let mat = geom
            .get("mat")
            .expect("quad has a mat")
            .as_str()
            .expect("mat is a str");
        let q = geom.get("q").expect("quad has a q");
        let q = parse_vec3(q);
        let u = geom.get("u").expect("quad has a u");
        let u = parse_vec3(u);
        let v = geom.get("v").expect("quad has a v");
        let v = parse_vec3(v);
        GeomDesc::Quad {
            q: q,
            u: u,
            v: v,
            mat: mat.to_string(),
        }
    } else if typ == "cube" {
        unimplemented!()
    } else if typ == "sphere" {
        let mat = geom
            .get("mat")
            .expect("sphere has a mat")
            .as_str()
            .expect("mat is a str");
        let c = geom.get("c").expect("sphere has a c");
        let c = parse_vec3(c);
        let r = geom
            .get("r")
            .expect("sphere has an r")
            .as_f64()
            .expect("r is a float");
        GeomDesc::Sphere {
            c: c,
            r: r,
            mat: mat.to_string(),
        }
    } else if typ == "translation" {
        unimplemented!()
    } else {
        unimplemented!()
    }
}

impl From<serde_json::Value> for SceneDesc {
    fn from(value: serde_json::Value) -> Self {
        let background_color = value
            .get("background_color")
            .expect("Scene has a background color");
        let background_color = parse_color(background_color);

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

        let geoms = value
            .get("geoms")
            .expect("scene has geoms")
            .as_array()
            .expect("geoms is an array");

        let geoms = geoms.iter().map(parse_geom).collect();

        SceneDesc {
            textures,
            materials,
            geoms,
            background_color,
        }
    }
}
