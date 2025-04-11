extern crate sdl2;

use lighting::color::Color;
use lighting::lambertian::Lambertian;
use lighting::metal::Metal;
use nalgebra::{Unit, Vector3};
use geom::intersectable::Intersectable;
use shape::plane::Plane;
use renderer::Renderer;
use scene::Scene;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use shape::sphere::Sphere;
use std::sync::Arc;
use std::time::Duration;

mod scene;
mod renderer;
mod shape;
mod geom;
mod lighting;
mod util;
mod sys;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 960;
    let window_height = 540;

    let window = video_subsystem.window("rust-sdl2 demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    let camera_pos = Vector3::new(0.0, 0.0, 50.0);
    let camera_dir= Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));
    let camera_down_dir = Unit::new_normalize(Vector3::new(0.0, -1.0, 0.0));
    let camera_right_dir = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
    let screen_dist = 50.0;
    let world_screen_width = 96.0;
    let world_screen_height= 54.0;

    let point8lambert = Lambertian::new(Color::new(0.8,0.8,0.8));
    let point8metal = Metal::new(Color::new(0.8, 0.8, 0.8));

    let ground = Arc::new(Plane::new(Vector3::new(0.0, -11.0, 0.0),Unit::new_normalize(Vector3::new(0.0, 1.0, -0.05)),Box::new(point8lambert)));
    let sphere0 = Arc::new(Sphere::new(Vector3::new(0.0, 0.0, -30.0), 20.0,Box::new(point8lambert)));
    let sphere1 = Arc::new(Sphere::new(Vector3::new(50.0, 0.0, -40.0), 15.0,Box::new(point8metal)));

    let objects : Vec<Arc<dyn Intersectable>> = vec![ground,sphere0,sphere1];

    let scene = Scene::new(objects);

    let recursion_depth = 50;
    let samples_per_pixel = 300;

    let renderer = Renderer::new(recursion_depth, window_width as u64, window_height as u64, camera_pos, camera_dir, camera_down_dir, camera_right_dir, screen_dist, world_screen_width, world_screen_height, samples_per_pixel);
    // let mut renderer = Renderer::new(canvas,window_width as u64,window_height as u64);

    renderer.render(&scene,&mut canvas);

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
