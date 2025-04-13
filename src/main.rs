extern crate sdl2;

use geom::intersectable::Intersectable;
use geom::plane::Plane;
use geom::sphere::Sphere;
use lighting::color::Color;
use lighting::lambertian::Lambertian;
use lighting::metal::Metal;
use lighting::texture::checkerboard::Checkerboard;
use lighting::texture::solidcolor::SolidColor;
use lighting::texture::Texture;
use nalgebra::{Unit, Vector3};
use rendering::renderer::Renderer;
use rendering::scene::Scene;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::Arc;
use std::time::Duration;

mod geom;
mod lighting;
mod rendering;
mod scenedesc;
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
    let camera_down_dir = Unit::new_normalize(Vector3::new(0.0, -1.0, 0.0));
    let camera_right_dir = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
    let screen_dist = 50.0;
    let world_screen_width = 96.0;
    let world_screen_height = 54.0;

    let point8solid : Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.8, 0.8, 0.8)));
    let point2solid : Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.2, 0.2, 0.2)));

    let point8lambert = Arc::new(Lambertian::new(point8solid.clone()));
    let point2lambert = Arc::new(Lambertian::new(point2solid.clone()));

    let checkertex : Arc<dyn Texture> = Arc::new(Checkerboard::new(5.0,point8solid.clone(),point2solid.clone()));

    let lambertchecker = Arc::new(Lambertian::new(checkertex));

    let point8metal = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.01));

    let ground = Box::new(Plane::new(
        Vector3::new(0.0, -11.0, 0.0),
        Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
        Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0)),
        lambertchecker
    ));
    let sphere0 = Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -30.0),
        20.0,
        point8lambert.clone(),
    ));
    let sphere1 = Box::new(Sphere::new(
        Vector3::new(50.0, 0.0, -40.0),
        15.0,
        point8metal,
    ));

    let objects: Vec<Box<dyn Intersectable>> = vec![ground, sphere0, sphere1];

    let scene = Scene::new(objects);

    let recursion_depth = 50;
    let samples_per_pixel = 300;

    let renderer = Renderer::new(
        recursion_depth,
        window_width as usize,
        window_height as usize,
        camera_pos,
        camera_dir,
        camera_down_dir,
        camera_right_dir,
        screen_dist,
        world_screen_width,
        world_screen_height,
        samples_per_pixel,
    );
    // let mut renderer = Renderer::new(canvas,window_width as u64,window_height as u64);

    let mut buf = renderer.render(&scene);
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
