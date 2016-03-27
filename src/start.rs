#![allow(unused_variables)]

extern crate orbclient;
extern crate sinulation;

#[cfg(target_os = "redox")]
use sinulation::Trig;

use super::vid;

/*cargo build --example tetrahedrane_example --target i386-unknown-redox.json -- -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code
*/

pub struct Window {
    pub screen_x: u32,
    pub screen_y: u32,

    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_z: f32,

    pub camera_x_y: f32,
    pub camera_x_z: f32,
    pub camera_y_z: f32,

    pub window: Box<orbclient::window::Window>,

    pub render_queue: Vec<vid::Triangle>,
}

impl Window {
    pub fn new(screen_x: u32, screen_y: u32, window_name: &str, triangle_space: usize) -> Window {
        let win = orbclient::window::Window::new_flags(10, 10, screen_x, screen_y, window_name, true).unwrap();
        Window {
            screen_x: screen_x,
            screen_y: screen_y,

            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: 0.0,

            camera_x_y: 0.0,
            camera_x_z: 0.0,
            camera_y_z: 0.0,

            window: win,

            render_queue: Vec::with_capacity(triangle_space),
        }
    }

    pub fn render(&mut self) {
        for mut triangle in &mut self.render_queue {
            let flat_1 = triangle.p1.flat_point(self.screen_x, self.screen_y, 
                                                triangle.x + self.camera_x, 
                                                triangle.y + self.camera_y,
                                                triangle.z + self.camera_z);
            let flat_2 = triangle.p2.flat_point(self.screen_x, self.screen_y,
                                                triangle.x + self.camera_x,
                                                triangle.y + self.camera_y,
                                                triangle.z + self.camera_z);
            let flat_3 = triangle.p3.flat_point(self.screen_x, self.screen_y,
                                                triangle.x + self.camera_x,
                                                triangle.y + self.camera_y,
                                                triangle.z + self.camera_z);
            
            self.window.line(flat_1.x, flat_1.y, flat_2.x, flat_2.y, triangle.color.orb_color());
            self.window.line(flat_2.x, flat_2.y, flat_3.x, flat_3.y, triangle.color.orb_color());
            self.window.line(flat_3.x, flat_3.y, flat_1.x, flat_1.y, triangle.color.orb_color());
        }

        self.render_queue = Vec::new();
    }

    pub fn push(&mut self, triangle: vid::Triangle) {
        self.render_queue.push(triangle);
    }

    pub fn push_group(&mut self, group: &vid::TriangleGroup) {
        for triangle in &group.triangles {
            self.push(triangle.clone());
        }
    }

    pub fn normalize_camera(&mut self) {
        #[cfg(not(target_os = "redox"))]
        use std::f32::consts::PI;

        #[cfg(target_os = "redox")]
        const PI: f32 = 3.141592653589793;

        if self.camera_x_z > (PI * 2.0) {
            self.camera_x_z -= (PI * 2.0);
        }

        if self.camera_x_y > (PI * 2.0) {
            self.camera_x_y -= (PI * 2.0);
        }

        if self.camera_y_z > (PI * 2.0) {
            self.camera_y_z -= (PI * 2.0);
        }
    }
}
