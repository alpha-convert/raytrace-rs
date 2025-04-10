extern crate sdl2;

use nalgebra::{Unit, Vector3};
use intersectable::Intersectable;
use plane::Plane;
use renderer::Renderer;
use scene::Scene;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sphere::Sphere;
use std::time::Duration;

mod scene;
mod renderer;
mod intersectable;
mod sphere;
mod plane;
mod ray;
mod light;
mod color;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 960;
    let window_height = 540;

    let window = video_subsystem.window("rust-sdl2 demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();

    let camera_pos = Vector3::new(0.0, 0.0, 50.0);
    let camera_dir= Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));
    let camera_down_dir = Unit::new_normalize(Vector3::new(0.0, -1.0, 0.0));
    let camera_right_dir = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
    let screen_dist = 100.0;
    let world_screen_width = 192.0;
    let world_screen_height= 108.0;

    let ground = Box::new(Plane::new(Vector3::new(0.0, -20.0, 0.0),Unit::new_normalize(Vector3::new(0.0, 1.0, -0.05))));
    let sphere0 = Box::new(Sphere::new(Vector3::new(0.0, 0.0, -30.0), 20.0,Color::RED));
    let sphere1 = Box::new(Sphere::new(Vector3::new(50.0, 0.0, -110.0), 10.0,Color::BLUE));

    let objects : Vec<Box<dyn Intersectable>> = vec![ground,sphere0,sphere1];

    let scene = Scene::new(objects);

    let samples_per_pixel = 1;

    let renderer = Renderer::new(window_width as u64, window_height as u64, canvas, camera_pos, camera_dir, camera_down_dir, camera_right_dir, screen_dist, world_screen_width, world_screen_height, samples_per_pixel);
    // let mut renderer = Renderer::new(canvas,window_width as u64,window_height as u64);

    renderer.render(&scene);

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
