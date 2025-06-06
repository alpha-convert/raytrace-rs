use std::{collections::HashMap, fs::File, sync::Arc};

use crate::{
    geom::{
        Geom, Geomable, aabb::AABB, bvh::BVH, cube::Cube, intersectable::Intersectable,
        intersection::Intersection, quad::Quad, sphere::Sphere, translation::Translation,
    },
    lighting::{
        color::Color,
        diffuselight::DiffuseLight,
        lambertian::Lambertian,
        material::Material,
        metal::Metal,
        texture::{
            Texture, checkerboard::Checkerboard, image::Image, scaletex::ScaleTex,
            solidcolor::SolidColor,
        },
    },
    math::{interval::Interval, ray::Ray},
};

use super::scenedesc::{GeomDesc, MaterialDesc, SceneDesc, TextureDesc};

pub struct Scene {
    bvh: BVH<Geom>,
    background_color: Color,
}

impl Scene {
    pub fn new(geoms: impl Geomable, background_color: Color) -> Self {
        let geoms = geoms.into_geoms().collect();
        Scene {
            bvh: BVH::construct(geoms),
            background_color,
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn bvh(&self) -> &BVH<Geom> {
        &self.bvh
    }
}

// fn construct_geom(gd: &GeomDesc, mat_map: &HashMap<String, Arc<dyn Material>>) -> Arc<dyn Intersectable> {
//     match gd {
//         GeomDesc::Cube { c, r, mat } => {
//             let mat = mat_map
//                 .get(mat)
//                 .expect(format!("material {} to be defined", mat).as_str())
//                 .clone();
//             Arc::new(Cube::new(*c, *r, mat))
//         }
//         // GeomDesc::Plane {
//         //     center,
//         //     normal,
//         //     u_hat,
//         //     v_hat,
//         //     mat,
//         // } => {
//         //     let mat = mat_map
//         //         .get(mat)
//         //         .expect(format!("material {} to be defined", mat).as_str())
//         //         .clone();
//         //     Arc::new(Plane::new(
//         //         *center,
//         //         Unit::new_normalize(*normal),
//         //         *u_hat,
//         //         *v_hat,
//         //         mat,
//         //     ))
//         // }
//         GeomDesc::Quad { q, u, v, mat } => {
//             let mat = mat_map
//                 .get(mat)
//                 .expect(format!("material {} to be defined", mat).as_str())
//                 .clone();
//             Arc::new(Quad::new(*q, *u, *v, mat))
//         }
//         GeomDesc::Sphere { c, r, mat } => {
//             let mat = mat_map
//                 .get(mat)
//                 .expect(format!("material {} to be defined", mat).as_str())
//                 .clone();
//             Arc::new(Sphere::new(*c, *r, mat))
//         }
//         GeomDesc::Translation { by, gd } => {
//             let geom = construct_geom(gd, mat_map);
//             Arc::new(Translation::new(*by, geom.into()))
//         }
//     }
// }

// impl<'a> From<&'a SceneDesc> for Scene {
//     fn from(sd: &'a SceneDesc) -> Self {
//         let mut tex_map: HashMap<String, Arc<dyn Texture>> = HashMap::new();

//         for (name, desc) in &sd.textures {
//             let tex: Arc<dyn Texture> = match desc {
//                 TextureDesc::Solid { albedo } => Arc::new(SolidColor::new(*albedo)),
//                 TextureDesc::Checkerboard {
//                     tex1,
//                     tex2,
//                     checker_size,
//                 } => {
//                     let tex1 = tex_map
//                         .get(tex1)
//                         .expect(format!("{} to be defined before {}", tex1, name).as_str())
//                         .clone();
//                     let tex2 = tex_map
//                         .get(tex2)
//                         .expect(format!("{} to be defined before {}", tex2, name).as_str())
//                         .clone();
//                     Arc::new(Checkerboard::new(*checker_size, tex1, tex2))
//                 }
//                 TextureDesc::Image { fname } => Arc::new(Image::from_fname(fname.as_str())),
//                 TextureDesc::ScaleTex {
//                     scale_u,
//                     scale_v,
//                     tex,
//                 } => {
//                     let tex = tex_map
//                         .get(tex)
//                         .expect(format!("{} to be defined before {}", tex, name).as_str())
//                         .clone();
//                     Arc::new(ScaleTex::new(*scale_u, *scale_v, tex))
//                 }
//             };

//             tex_map.insert(name.clone(), tex);
//         }

//         let mut mat_map: HashMap<String, Arc<dyn Material>> = HashMap::new();

//         for (name, desc) in &sd.materials {
//             let mat: Arc<dyn Material> = match desc {
//                 MaterialDesc::DiffuseLight { tex } => {
//                     let tex = tex_map
//                         .get(tex)
//                         .expect(format!("{} to be defined before {}", tex, name).as_str())
//                         .clone();
//                     Arc::new(DiffuseLight::new(tex))
//                 }
//                 MaterialDesc::Lambertian { tex } => {
//                     let tex = tex_map
//                         .get(tex)
//                         .expect(format!("{} to be defined before {}", tex, name).as_str())
//                         .clone();
//                     Arc::new(Lambertian::new(tex))
//                 }
//                 MaterialDesc::Metal { albedo, fuzz } => Arc::new(Metal::new(*albedo, *fuzz)),
//             };

//             mat_map.insert(name.clone(), mat);
//         }

//         let mut geoms: Vec<Arc<dyn Intersectable>> = Vec::with_capacity(sd.geoms.len());

//         for geom_desc in &sd.geoms {
//             geoms.push(construct_geom(geom_desc, &mat_map));
//         }

//         let geoms = BVH::construct(geoms);

//         Scene {
//             geoms: geoms,
//             background_color: sd.background_color,
//         }
//     }
// }
