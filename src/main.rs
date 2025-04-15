extern crate sdl2;

use geom::Geom;
use geom::quad::Quad;
use geom::scaling::Scaling;
use geom::sphere::Sphere;
use geom::translation::Translation;
use geom::triangle::Triangle;
use geom::trimesh::TriMesh;
use lighting::color::Color;
use lighting::diffuselight::DiffuseLight;
use lighting::lambertian::Lambertian;
use lighting::material::Material;
use lighting::metal::Metal;
use lighting::texture::Texture;
use lighting::texture::image::Image;
use lighting::texture::solidcolor::SolidColor;
use nalgebra::{Unit, Vector3};
use rendering::camera::Camera;
use rendering::renderer::Renderer;
use rendering::scene::Scene;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::Arc;
use std::time::Duration;

mod geom;
mod lighting;
mod math;
mod rendering;
mod util;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 960;
    let window_height = 540;

    let window = video_subsystem
        .window("rust-sdl2 demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let camera_pos = Vector3::new(0.0, 0.0, 50.0);
    let camera_dir = Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));
    let camera_up_dir = Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0));
    let screen_dist = 50.0;
    let world_screen_width = 96.0;
    let world_screen_height = 54.0;

    let point8solid: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.2, 0.2, 0.8)));
    let point2solid: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.2, 0.2, 0.2)));

    let birdttex: Arc<dyn Texture> = Arc::new(Image::from_fname("bird.jpeg"));
    let birdlight = Arc::new(DiffuseLight::new(birdttex));

    let whitediffuse: Arc<dyn Material> = Arc::new(DiffuseLight::solid(Color::new(0.1, 0.55, 0.2)));

    let point8lambert = Arc::new(Lambertian::new(point8solid.clone()));

    // let blendermonkey = Arc::new(Translation::new(
    //     Vector3::new(0.0, 0.0, 45.0),
    //     Arc::new(TriMesh::from_fname(
    //         "scenes/blender.obj",
    //         point8lambert.clone(),
    //     )),
    // ));

    let bunny = Arc::new(Scaling::new(
        Vector3::new(200.0, 200.0, 200.0),
        Arc::new(TriMesh::from_fname(
            "scenes/bunny.obj",
            point8lambert.clone(),
        )),
    ));

    // let checkertex: Arc<dyn Texture> =
    // Arc::new(Checkerboard::new(
    //     0.2,
    //     point8solid.clone(),
    //     point2solid.clone(),
    // ));

    // let lambertchecker = Arc::new(Lambertian::new(checkertex));

    let point8metal = Arc::new(Metal::new(Color::new(0.9, 0.8, 0.8), 0.01));

    let sqr = Arc::new(Quad::new(
        Vector3::new(2.5, -2.0, 46.0),
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 10.0, 0.0),
        birdlight.clone(),
    ));

    let sqr2 = Arc::new(Quad::new(
        Vector3::new(-2.5, -2.0, 46.0),
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 10.0, 0.0),
        birdlight.clone(),
    ));

    // let sqr2 = Box::new(Quad::new(
    //     Vector3::new(0.0, 5.0, 5.0),
    //     Vector3::new(5.0, 0.0, 0.0),
    //     Vector3::new(0.0, 0.0, -5.0),
    //     point8metal.clone()
    // ));

    let ground = Arc::new(Quad::new(
        Vector3::new(-20.0, -12.0, 0.0),
        Vector3::new(40.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -30.0),
        birdlight.clone(),
    ));
    // let ground = Arc::new(Plane::new(
    //     Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
    // ));
    let sphere0 = Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, -30.0),
        20.0,
        point8lambert.clone(),
    ));
    let sphere1 = Arc::new(Sphere::new(
        Vector3::new(50.0, 0.0, -40.0),
        15.0,
        point8metal,
    ));

    let tri0 = Arc::new(Triangle::new(
        Vector3::new(10.0, 10.0, 0.0),
        Vector3::new(-10.0, 0.0, 0.0),
        Vector3::new(-10.0, 10.0, 0.0),
        Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
        birdlight.clone(),
    ));

    let objects: Vec<Arc<dyn Geom>> = vec![bunny, sqr, sqr2, ground, sphere1];

    let scene = Scene::new(objects, Color::white());

    let recursion_depth = 50;
    let samples_per_pixel = 300;

    let camera = Camera::new(
        window_width as usize,
        window_height as usize,
        camera_pos,
        camera_dir,
        camera_up_dir,
        screen_dist,
        80.0,
    );

    let renderer = Renderer::new(
        recursion_depth,
        window_width as usize,
        window_height as usize,
        samples_per_pixel,
    );
    // let mut renderer = Renderer::new(canvas,window_width as u64,window_height as u64);

    let mut buf = renderer.render(&camera, &scene);
    buf.blit_to(&mut canvas);
    canvas.present();

    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
