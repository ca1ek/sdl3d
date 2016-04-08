extern crate orbclient;

use super::render;

pub struct Window {
    width: u32,
    height: u32,
    window: Box<orbclient::window::Window>
}

impl Window {
    pub fn new(x: i32, y: i32, width: u32, height: u32, title: &str) -> Window {
        Window {
            width: width,
            height: height,
            window: Box::new(orbclient::window::Window::new_flags(x, y, width, height, title, true)).unwrap()
        }
    }

    pub fn apply_buf(&mut self, framebuffer: &render::Framebuffer) {
        for x in 0..framebuffer.width as u32 {
            for y in 0..framebuffer.height as u32 {
                let pixel = framebuffer.get_pixel(x, y);

                if pixel.is_none() {
                    continue;
                } else {
                    self.window.pixel(x as i32, y as i32, pixel.unwrap().as_orbclient());
                }
            }
        }
    }

    pub fn sync(&mut self) {
        self.window.sync();
    }
}
