use nalgebra::{Unit, Vector3};

use crate::math::ray::Ray;

pub struct Camera {
    pos: Vector3<f64>,

    //World-position screen data
    pixel_00_center: Vector3<f64>, //world space location of the center of (0,0) in pixel space. (i.e) the screen
    pixel_delta_u: Vector3<f64>,   //how much to the right in world space +1px in screen space x is
    pixel_delta_v: Vector3<f64>,   //how much down in world space +1px in screen space y is
}

impl Camera {
    pub fn new(
        window_width: usize,
        window_height: usize,
        pos: Vector3<f64>,
        fwd: Unit<Vector3<f64>>,
        up: Unit<Vector3<f64>>,
        focal_length: f64,
        vfov: f64,
        // world_screen_width: f64,
        // world_screen_height: f64,
    ) -> Self {
        let theta = f64::to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (window_width as f64 / window_height as f64);

        let lookat = pos + fwd.scale(focal_length);

        // let idx_scale_factor_x = world_screen_width / (window_width as f64);
        // let idx_scale_factor_y = world_screen_height / (window_height as f64);

        let w = Unit::new_normalize(pos - lookat);
        let u = fwd.cross(&up);
        let v = w.cross(&u);

        let viewport_u = u.scale(viewport_width);
        let viewport_v = -v.scale(viewport_height);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u.scale(1.0 / window_width as f64);
        let pixel_delta_v = viewport_v.scale(1.0 / window_height as f64);

        // let pixel_delta_u = u.scale(idx_scale_factor_x);
        // let pixel_delta_v = -up.scale(idx_scale_factor_y);
        let viewport_upper_left =
            pos - w.scale(focal_length) - viewport_u.scale(0.5) - viewport_v.scale(0.5);
        let screen00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        // let screen00 = lookat
        // - camera_right.scale(world_screen_width / 2.0)
        // + up.scale(world_screen_height / 2.0)
        // + pixel_delta_u.scale(0.5)
        // + pixel_delta_v.scale(0.5);

        Camera {
            pos: pos,
            pixel_00_center: screen00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_through(&self, u: f64, v: f64) -> Ray {
        let pt = self.pixel_00_center + self.pixel_delta_u.scale(u) + self.pixel_delta_v.scale(v);
        Ray::through_points(self.pos, pt)
    }
}
