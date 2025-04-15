use nalgebra::{Unit, Vector3};

use crate::math::ray::Ray;

pub struct Camera {
    pos: Vector3<f64>,

    //World-position screen data
    screen_00: Vector3<f64>, //world space location of the center of (0,0) in pixel space. (i.e) the screen
    screen_delta_u: Vector3<f64>, //how much to the right in world space +1px in screen space x is
    screen_delta_v: Vector3<f64>, //how much down in world space +1px in screen space y is
}

impl Camera {
    pub fn new(
        window_width: usize,
        window_height: usize,
        pos: Vector3<f64>,
        camera_fwd: Unit<Vector3<f64>>,
        camera_down: Unit<Vector3<f64>>,
        camera_right: Unit<Vector3<f64>>,
        screen_dist: f64,
        world_screen_width: f64,
        world_screen_height: f64,
    ) -> Self {
        let idx_scale_factor_x = world_screen_width / (window_width as f64);
        let idx_scale_factor_y = world_screen_height / (window_height as f64);
        // let camera_right = Unit::new_normalize(camera_fwd.cross(&camera_down));
        let screen_delta_u = camera_right.scale(idx_scale_factor_x);
        let screen_delta_v = camera_down.scale(idx_scale_factor_y);
        let screen_center_world = pos + camera_fwd.scale(screen_dist);
        let screen00 = screen_center_world
            - camera_right.scale(world_screen_width / 2.0)
            - camera_down.scale(world_screen_height / 2.0)
            + screen_delta_u.scale(0.5)
            + screen_delta_v.scale(0.5);

        Camera {
            pos: pos,
            screen_00: screen00,
            screen_delta_u: screen_delta_u,
            screen_delta_v: screen_delta_v,
        }
    }

    pub fn ray_through(&self, u: f64, v: f64) -> Ray {
        let pt = self.screen_00 + self.screen_delta_u.scale(u) + self.screen_delta_v.scale(v);
        Ray::through_points(self.pos, pt)
    }
}
