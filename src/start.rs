#![allow(unused_variables)]

extern crate orbclient;
extern crate sinulation;

#[cfg(target_os = "redox")]
use sinulation::Trig;

use super::vid;

// Had to store this somewhere.
/*cargo build --example tetrahedrane_example --target i386-unknown-redox.json -- -C no-prepopulate-passes -C no-stack-check -C opt-level=2 -Z no-landing-pads -A dead_code
*/

/// Stores data about rendering, window and camera.
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
    /// Create a new window.
    ///
    /// * `triangle_space` - how much space to preallocate for the triangles
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

    /// Renders triangles onto the framebuffer.
    pub fn render(&mut self, triangle: vid::Triangle, shaders: &Vec<vid::Shader>) {
        for shader_id in triangle.shader_ids.clone().iter() {
            let mut assoc_shader = shaders.iter().find(|&shader| shader.id == shader_id.clone());
            if assoc_shader.is_none() {
                continue;
            }

            let mut unwrapped_shader = assoc_shader.unwrap();
            (unwrapped_shader.shader)(&triangle, self, unwrapped_shader);
        }

        let used_space = self.render_queue.len();

        self.render_queue = Vec::with_capacity(used_space);
    }

    pub fn render_addon_shader(&mut self, triangle: vid::Triangle, shaders: &Vec<vid::Shader>, additional_id: [u16; 8]) {
        for shader_id in triangle.shader_ids.clone().iter() {
            let mut assoc_shader = shaders.iter().find(|&shader| shader.id == shader_id.clone());
            if assoc_shader.is_none() {
                continue;
            }

            let mut unwrapped_shader = assoc_shader.unwrap();
            (unwrapped_shader.shader)(&triangle, self, unwrapped_shader);
        }

        for shader_id in additional_id.iter() {
            let mut assoc_shader = shaders.iter().find(|&shader| shader.id == shader_id.clone());
            if assoc_shader.is_none() {
                continue;
            }

            let mut unwrapped_shader = assoc_shader.unwrap();
            (unwrapped_shader.shader)(&triangle, self, unwrapped_shader);
        }

        let used_space = self.render_queue.len();

        self.render_queue = Vec::with_capacity(used_space);
    }

    pub fn render_group(&mut self, group: vid::TriangleGroup, shaders: &Vec<vid::Shader>) {
        let group_shaders = group.shader_ids.clone();
        for triangle in group.triangles {
            self.render_addon_shader(triangle, shaders, group_shaders);
        }
    }


    /// Push a triangle onto the render queue.
    pub fn push(&mut self, triangle: vid::Triangle) {
        self.render_queue.push(triangle);
    }

    /// Push a group of triangles onto the render queue.
    pub fn push_group(&mut self, group: &vid::TriangleGroup) {
        for triangle in &group.triangles {
            self.push(triangle.clone());
        }
    }

    /// Normalize the camera rotations.
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
