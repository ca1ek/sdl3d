#![allow(unused_variables)]

extern crate sdl2;

use super::vid;

/*struct SDL_handles<'a> {
    context: sdl2::sdl::Sdl,
    video: sdl2::sdl::VideoSubsystem,
    renderer: sdl2::render::Renderer<'a>,
}*/

pub struct Engine<'a> {
    pub screen_x: u32,
    pub screen_y: u32,

    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_z: f32,

    pub camera_x_y: f32,
    pub camera_x_z: f32,
    pub camera_y_z: f32,

    pub renderer: sdl2::render::Renderer<'a>,

    pub event_pump: sdl2::EventPump,

    pub render_queue: Vec<vid::Triangle>,
}

impl<'a> Engine<'a> {
    pub fn new(screen_x: u32, screen_y: u32, window_name: String, triangle_space: usize) -> Engine<'a> {
        let sdl_ctx = sdl2::init().unwrap();
        let sdl_vid = sdl_ctx.video().unwrap();

        let sdl_win = sdl_vid.window(&window_name, screen_x, screen_y)
                        .position_centered()
                        .opengl()
                        .build()
                        .expect("Failed on creating a new window!");
        Engine {
            screen_x: screen_x,
            screen_y: screen_y,

            camera_x: 0.0,
            camera_y: 0.0,
            camera_z: 0.0,

            camera_x_y: 0.0,
            camera_x_z: 0.0,
            camera_y_z: 0.0,

            renderer: sdl_win.renderer().build().unwrap(),

            event_pump: sdl_ctx.event_pump().unwrap(),

            render_queue: Vec::with_capacity(triangle_space),
        }
    }

    pub fn render(&mut self) {
        for mut triangle in &mut self.render_queue {
            let flat_1 = triangle.p1.flat_point(self.screen_x, self.screen_y, 
                                                triangle.x + self.camera_x, 
                                                triangle.y + self.camera_y,
                                                triangle.z + self.camera_z).make_sdl();
            let flat_2 = triangle.p2.flat_point(self.screen_x, self.screen_y,
                                                triangle.x + self.camera_x,
                                                triangle.y + self.camera_y,
                                                triangle.z + self.camera_z).make_sdl();
            let flat_3 = triangle.p3.flat_point(self.screen_x, self.screen_y,
                                                triangle.x + self.camera_x,
                                                triangle.y + self.camera_y,
                                                triangle.z + self.camera_z).make_sdl();
            
            self.renderer.draw_lines(&[flat_1, flat_2, flat_3, flat_1]);
        }

        self.render_queue = Vec::new();
    }

    pub fn normalize_camera(&mut self) {
        use std::f32::consts::PI;

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


pub fn bootstrap<'a>(win_width: i32, win_height: i32, win_name: &str) -> (sdl2::render::Renderer<'a>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap(); // context
    let sdl_video = sdl_context.video().unwrap(); // video
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Make a new window
    let window = sdl_video.window(win_name, 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed on creating a new window!");
    
    // turn window into a renderer, cannot do anything with window from now on.
    let mut renderer = window.renderer().build().unwrap(); 

    (renderer, event_pump)
}
