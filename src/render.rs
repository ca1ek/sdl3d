extern crate orbclient;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn as_orbclient(&self) -> orbclient::color::Color {
        orbclient::color::Color::rgb(self.r, self.g, self.b)
    }

    pub fn from_orbclient(color: orbclient::color::Color) -> Color {
        let r = (color.data & 0x00FF0000) >> 16;
        let g = (color.data & 0x0000FF00) >> 8;
        let b = color.data & 0x000000FF;

        Color::new(r as u8, g as u8, b as u8)
    }
}

pub struct Texture {
    pub width: usize,
    pub height: usize,
    frame: Vec<Color>,
}

impl Texture {
    pub fn new(w: usize, h: usize) -> Texture {
        let mut tex: Vec<Color> = Vec::new();

        for _ in 0..w*h {
            tex.push(Color::new(0, 0, 0));
        }

        Texture {
            width: w,
            height: h,
            frame: tex,
        }
    }

    pub fn load(path: &str) -> Texture {
        use std::ops::Deref;

        let bmp = orbclient::bmp::BmpFile::from_path(path);
        let w = bmp.width();
        let h = bmp.height();

        let orb_color_vec = bmp.deref();

        let mut color_vec: Vec<Color> = Vec::new();

        for i in 0..orb_color_vec.len() {
            color_vec.push(Color::from_orbclient(orb_color_vec[i]));
        }

        Texture {
            width: w,
            height: h,
            frame: color_vec,
        }
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    frame: Vec<Color>,
}

impl Framebuffer {
    pub fn new(w: usize, h: usize) -> Framebuffer {
        let mut frame: Vec<Color> = Vec::new();

        for _ in 0..w*h {
            frame.push(Color::new(0, 0, 0));
        }

        Framebuffer {
            width: w,
            height: h,
            frame: frame,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&Color> {
        self.frame.get(y as usize*self.width + x as usize)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        self.frame[y as usize*self.width + x as usize] = color.clone();
    }

    pub fn render_triangles(&mut self, triangles: Vec<super::geometry::Triangle>) {

    }
}
