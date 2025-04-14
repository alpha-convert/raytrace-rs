use std::{collections::HashMap, sync::Arc};

use crate::{geom::{
    intersectable::{Intersectable, Intersection},
    interval::Interval,
    ray::Ray,
}, lighting::{color::Color, diffuselight::DiffuseLight, lambertian::Lambertian, material::Material, metal::Metal, texture::{checkerboard::Checkerboard, image::Image, scaletex::ScaleTex, solidcolor::SolidColor, Texture}}};

use super::scenedesc::{GeomDesc, MaterialDesc, SceneDesc, TextureDesc};

pub struct Scene {
    geoms : Vec<Box<dyn Intersectable>>,
    background_color : Color,
}

impl Scene {
    pub fn new(geoms : Vec<Box<dyn Intersectable>>, background_color : Color) -> Self {
        Scene { geoms : geoms , background_color }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Intersectable for Scene {
    fn intersect<'r>(&'r self, ray: &'r Ray, i: Interval) -> Option<Intersection<'r>> {
        self.geoms.iter().filter_map(|obj| { obj.intersect(ray, i)}).min_by(Intersection::dist_compare)
    }
}


impl<'a> From<&'a SceneDesc> for Scene {
    fn from(sd : &'a SceneDesc) -> Self {
        let tex_desc_map = sd.textures;
        let mat_desc_map = sd.materials;

        let mut tex_map: HashMap<String, Arc<dyn Texture>> = HashMap::new();

        for (name,desc) in tex_desc_map {
            let tex : Arc<dyn Texture> = match desc {
                TextureDesc::Solid { albedo } => Arc::new(SolidColor::new(albedo)),
                TextureDesc::Checkerboard { tex1, tex2, checker_size } => {
                    let tex1 = tex_map.get(&tex1).expect(format!("{} to be defined before {}",tex1, name).as_str()).clone();
                    let tex2 = tex_map.get(&tex2).expect(format!("{} to be defined before {}",tex2, name).as_str()).clone();
                    Arc::new(Checkerboard::new(checker_size, tex1, tex2))
                },
                TextureDesc::Image { fname } => {
                    Arc::new(Image::from_fname(fname.as_str()))
                },
                TextureDesc::ScaleTex { scale_u, scale_v, tex } => {
                    let tex = tex_map.get(&tex).expect(format!("{} to be defined before {}",tex,name).as_str()).clone();
                    Arc::new(ScaleTex::new(scale_u,scale_v,tex))
                },
            };

            tex_map.insert(name,tex);
        }


        let mut mat_map = HashMap::new();

        for (name,desc) in mat_desc_map {
            let mat : Arc<dyn Material> = match desc {
                MaterialDesc::DiffuseLight { tex } => {
                    let tex = tex_map.get(&tex).expect(format!("{} to be defined before {}",tex,name).as_str()).clone();
                    Arc::new(DiffuseLight::new(tex))
                },
                MaterialDesc::Lambertian { tex } => {
                    let tex = tex_map.get(&tex).expect(format!("{} to be defined before {}",tex,name).as_str()).clone();
                    Arc::new(Lambertian::new(tex))
                },
                MaterialDesc::Metal { albedo, fuzz } => {
                    Arc::new(Metal::new(albedo, fuzz))
                },
            };

            mat_map.insert(name,mat);
        };

        let mut geoms : Vec<Box<dyn Intersectable>> = Vec::with_capacity(sd.geoms.len());

        for geom_desc in &sd.geoms {
            match geom_desc {
                GeomDesc::Cube { c, r, mat } => todo!(),
                GeomDesc::Plane { center, normal, u_hat, v_hat, mat } => todo!(),
                GeomDesc::Quad { q, u, v, mat } => todo!(),
                GeomDesc::Sphere { c, r, mat } => todo!(),
                GeomDesc::Translation { by, geom } => todo!(),
            }
        };

        Scene { geoms: geoms, background_color: sd.background_color }


    }
}
