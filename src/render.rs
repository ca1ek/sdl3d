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
