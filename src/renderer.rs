use std::cell::RefCell;

use nalgebra::{Unit, Vector2, Vector3};
use rand::Rng;
use sdl2::{render::Canvas, video::Window};

use crate::{color::Color, intersectable::{Intersectable, Intersection}, ray::Ray, scene::Scene};

pub struct Renderer {
    //Canvas data
    window_width : u64,
    window_height : u64,
    canvas : RefCell<Canvas<Window>>,

    //Camera data
    pub camera_pos : Vector3<f64>,

    //World-position screen data
    screen_00 : Vector3<f64>, //world space location of coords (0,0) in pixel space. (i.e) the screen
    screen_delta_u : Vector3<f64>, //how much to the right in world space +1px in screen space x is
    screen_delta_v : Vector3<f64>,  //how much down in world space +1px in screen space y is

    //AA data
    samples_per_pixel : u64,
    sample_weight : f64, //equals 1/samples_per_pixel
}

impl Renderer {
    pub fn new(window_width: u64,
                window_height : u64,
                canvas : Canvas<Window>,

                camera_pos : Vector3<f64>,
                camera_fwd : Unit<Vector3<f64>>,
                camera_down : Unit<Vector3<f64>>,
                camera_right : Unit<Vector3<f64>>,
                screen_dist : f64,
                world_screen_width : f64,
                world_screen_height : f64,
                samples_per_pixel : u64
            ) -> Self {

        let idx_scale_factor_x = world_screen_width / (window_width as f64);
        let idx_scale_factor_y = world_screen_height / (window_height as f64);
        // let camera_right = Unit::new_normalize(camera_fwd.cross(&camera_down));
        let screen_delta_u = camera_right.scale(idx_scale_factor_x);
        let screen_delta_v = camera_down.scale(idx_scale_factor_y);
        let screen_center_world = camera_pos + camera_fwd.scale(screen_dist);
        let screen00 = screen_center_world - camera_right.scale(world_screen_width/2.0) -  camera_down.scale(world_screen_height/2.0);

        let sample_weight = 1.0 / (samples_per_pixel as f64);

        Renderer { 
            window_width: window_width,
            window_height: window_height,
            canvas: RefCell::new(canvas),
            camera_pos,
            // camera_dir: camera_fwd,
            // camera_down_dir: camera_down,
            // camera_right_dir: camera_right,
            screen_00: screen00,
            screen_delta_u,
            screen_delta_v, 
            samples_per_pixel,
            sample_weight
        }
    }

    fn sample_uv() -> (f64,f64) {
        let mut rng = rand::rng();
        let du : f64 = rng.random::<f64>() - 0.5;
        let dv : f64 = rng.random::<f64>() - 0.5;
        assert!(-0.5 <= du && du <= 0.5);
        assert!(-0.5 <= dv && dv <= 0.5);
        (du,dv)
    }

    pub fn render(&self, scene : &Scene) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::new(0.0, 0.0,0.0));
        canvas.clear();


        for x_idx in 0..self.window_width {
            for y_idx in 0..self.window_height {

                let mut px_color = Color::new(0.0,0.0,0.0);

                for _ in 0..self.samples_per_pixel {
                    let (du,dv) = Renderer::sample_uv();
                    let screen_point = 
                        self.screen_00 + self.screen_delta_u.scale(x_idx as f64 + du) + self.screen_delta_v.scale(y_idx as f64 + dv);

                    let ray = Ray::through_points(self.camera_pos,screen_point);

                    px_color = px_color + self.trace(&ray, scene).scale(self.sample_weight);
                }
                
                 canvas.set_draw_color(px_color);
                 canvas.draw_point((x_idx as i32,y_idx as i32)).expect("Should be able to draw a point.");
                

            }
        }

        canvas.present();
    }

    fn trace(&self, ray : &Ray, scene : &Scene) -> Color {
        if let Some(inter) = scene.intersect(&ray) {
            Renderer::shade(&inter)
        } else {
            Color::white()
        }

    }

    fn shade(inter : &Intersection) -> Color {
        let r = inter.normal().x.abs();
        let g = inter.normal().y.abs();
        let b = inter.normal().z.abs();

        Color::new(r, g, b)
    }
}