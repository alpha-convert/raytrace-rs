use std::cell::RefCell;

use nalgebra::{Unit, Vector3};
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::{intersectable::{Intersectable, Intersection}, ray::Ray, scene::Scene};

pub struct Renderer {
    //Canvas data
    window_width : u64,
    window_height : u64,
    canvas : RefCell<Canvas<Window>>,

    //Camera data
    pub camera_pos : Vector3<f64>,
    pub camera_dir : Unit<Vector3<f64>>,
    camera_down_dir : Unit<Vector3<f64>>,
    camera_right_dir : Unit<Vector3<f64>>,

    //World-position screen data
    screen_00 : Vector3<f64>, //world space location of coords (0,0) in pixel space. (i.e) the screen
    screen_delta_u : Vector3<f64>, //how much to the right in world space +1px in screen space x is
    screen_delta_v : Vector3<f64>  //how much down in world space +1px in screen space y is

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
                world_screen_height : f64
            ) -> Self {

        let idx_scale_factor_x = world_screen_width / (window_width as f64);
        let idx_scale_factor_y = world_screen_height / (window_height as f64);
        // let camera_right = Unit::new_normalize(camera_fwd.cross(&camera_down));
        let screen_delta_u = camera_right.scale(idx_scale_factor_x);
        let screen_delta_v = camera_down.scale(idx_scale_factor_y);
        let screen_center_world = camera_pos + camera_fwd.scale(screen_dist);
        let screen00 = screen_center_world - camera_right.scale(world_screen_width/2.0) -  camera_down.scale(world_screen_height/2.0);

        Renderer { 
            window_width: window_width,
            window_height: window_height,
            canvas: RefCell::new(canvas),
            camera_pos,
            camera_dir: camera_fwd,
            camera_down_dir: camera_down,
            camera_right_dir: camera_right,
            screen_00: screen00,
            screen_delta_u,
            screen_delta_v, 
        }
    }

    pub fn render(&self, scene : &Scene) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();


        for x_idx in 0..self.window_width {
            for y_idx in 0..self.window_height {
                
                let screen_point = 
                    self.screen_00 + self.screen_delta_u.scale(x_idx as f64) + self.screen_delta_v.scale(y_idx as f64);

                let ray = Ray::through_points(self.camera_pos,screen_point);

                
                let objs : &Vec<Box<dyn Intersectable>> = &scene.objects;

                if let Some(inter) = objs.intersect(&ray) {
                    let color = Renderer::shade(&inter);

                    canvas.set_draw_color(color);
                    canvas.draw_point((x_idx as i32,y_idx as i32)).expect("Should be able to draw a point.");
                }

            }
        }

        canvas.present();
    }

    fn shade<'r>(inter : &Intersection<'r>) -> Color {
        let r = (inter.normal().x * 255.0) as u8;
        let g = (inter.normal().y * 255.0) as u8;
        let b = (inter.normal().z * 255.0) as u8;

        Color::RGB(r, g, b)
    }
}