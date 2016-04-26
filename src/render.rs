extern crate orbclient;

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
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

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&Color> {
        self.frame.get(y as usize*self.width + x as usize)
    }

    pub fn get_by_float(&self, x: f32, y: f32) -> Option<&Color> {
        self.get_pixel((x * self.width as f32) as u32, (y * self.height as f32) as u32)
    }

    pub fn get_for_triangle(&self, alpha: f32, beta: f32, gamma: f32,
                            triangle: &super::geometry::Triangle) -> Option<&Color> {
        self.get_by_float(triangle.uv_p1.x * alpha + triangle.uv_p2.x * beta + triangle.uv_p3.x * gamma,
                          triangle.uv_p1.y * alpha + triangle.uv_p2.y * beta + triangle.uv_p3.y * gamma)
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

    pub fn render_3d_triangles(&mut self, triangles: &Vec<&super::geometry::Triangle>, screen_h: i32) {
        let mut flat_triangles: Vec<super::geometry::FlatTriangle> = Vec::new();
        for triangle in triangles {
            flat_triangles.push(triangle.make_2d())
        }

        for x in 0..self.width {
            for y in 0..self.height {
                for triangle in &flat_triangles {
                    //self.set_pixel(triangle.p1.screen_point(), triangle.p1.y, &Color::new(255, 255, 255));
                    if triangle.inside(x as u32, y as u32, screen_h) {
                        self.set_pixel(x as u32, y as u32, &Color::new(255, 0, 0));
                        //print!("inside");
                    } else {
                        self.set_pixel(x as u32, y as u32, &Color::new(0, 0, 0));
                    }
                }
            }
        }
    }
}
