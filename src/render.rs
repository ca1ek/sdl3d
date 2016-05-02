extern crate orbclient;

use std;

#[derive(Clone, Copy, Debug)]
/// An RGB color
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

#[derive(Clone, Copy, Debug)]
/// A color with Z depth data attached
pub struct ZBufColor {
    pub color: Color,
    pub z: f32,
}

impl ZBufColor {
    pub fn new(r: u8, g: u8, b: u8, z: f32) -> ZBufColor {
        ZBufColor {
            color: Color {
                r: r,
                g: g,
                b: b,
            },
            z: z,
        }
    }

    pub fn from_color(color: Color, z: f32) -> ZBufColor {
        ZBufColor {
            color: color,
            z: z,
        }
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

//FIXME: This was a big mistake. Go back to rendering right to the output instead of some
// wrapper structs like this.
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    cache: Vec<ZBufColor>, // workaround around orbclients lack of ability to read from the screen
    frame: super::init::Window,
}

impl Framebuffer {
    pub fn new(w: usize, h: usize, window: super::init::Window) -> Framebuffer {
        let mut cache: Vec<ZBufColor> = Vec::new();

        for _ in 0..w*h {
            cache.push(ZBufColor::new(0, 0, 0, std::f32::INFINITY));
        }

        Framebuffer {
            width: w,
            height: h,
            cache: cache,
            frame: window,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&ZBufColor> {
        self.cache.get(y as usize*self.width + x as usize)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &ZBufColor) {
        //self.cache[y as usize*self.width + x as usize] = color.clone(); // cache
        self.frame.set_pixel(x as i32, y as i32, &color.color); // draw
    }

    pub fn clear(&mut self) {
        for pixel in 0..self.width*self.height {
            self.cache[pixel] = ZBufColor::new(0, 0, 0, std::f32::INFINITY); // cache
        }
        self.frame.clear(); // draw
    }

    pub fn sync(&mut self) {
        self.frame.sync();
    }

    pub fn render_3d_triangles(&mut self, triangles: &Vec<&super::geometry::Triangle>, screen_h: i32, screen_w: i32) {
        /*let mut flat_triangles: Vec<super::geometry::FlatTriangle> = Vec::new();
        for triangle in triangles {
            flat_triangles.push(triangle.make_2d())
        }

        for x in 0..self.width {
            for y in 0..self.height {
                for triangle in &flat_triangles {
                    //self.set_pixel(triangle.p1.screen_point(), triangle.p1.y, &Color::new(255, 255, 255));
                    if triangle.inside(x as u32, y as u32, screen_h) {
                        self.set_pixel(x as u32, y as u32, &ZBufColor::new(255, 0, 0, 0.0));
                        //TODO: Use real Z value.
                        //print!("inside");
                    } else {
                        self.set_pixel(x as u32, y as u32, &ZBufColor::new(0, 0, 0, 0.0));
                    }
                }
            }
        }*/

        for x in 0..self.width {
            for y in 0..self.height {
                for triangle in triangles {
                    let flat_triangle = triangle.make_2d();

                    let (alpha, beta, gamma) = flat_triangle.get_barycentric(x as u32, y as u32, screen_h, screen_w);

                    if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
                        let color_opt = triangle.texture.get_for_triangle(alpha, beta, gamma, &triangle);
                        let mut color = Color::new(0,0,0);
                        if !color_opt.is_none() {
                            color = color_opt.unwrap().clone();
                        }
                        self.set_pixel(x as u32, y as u32, 
                            &ZBufColor::from_color(color,
                                        triangle.z_from_barycentric(alpha, beta, gamma)));
                    } else {
                        //self.set_pixel(x as u32, y as u32, &ZBufColor::new(0, 0, 0, 0.0));
                    }
                }
            }
        }
    }

    pub fn draw_cached_zbuf(&mut self) {
        for x in 0..self.width as u32 {
            for y in 0..self.height as u32 {
                let pixel = self.cache.get(y as usize*self.width + x as usize);

                if pixel.is_none() {
                    continue;
                } else {
                    let z_color = (((1.0 / -pixel.unwrap().z) * 255.0) + 255.0) as u8;
                    &self.frame.set_pixel(x as i32, y as i32, &Color::new(z_color, z_color, z_color));
                }
            }
        }
    }
}
