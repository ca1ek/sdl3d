use orbclient;
use super::super::geometry;
use super::super::texture;
use std;

pub struct ZBuffer {
    w: u32,
    h: u32,
    data: Box<[f32]>
}

impl ZBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        use std::f32;
        ZBuffer {
            w: width,
            h: height,
            data: vec![f32::INFINITY; width as usize * height as usize].into_boxed_slice()
        }
    }
    
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        self.data[x as usize + (self.w * y) as usize] = value;
    }
    
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[x as usize + (self.w * y) as usize]
    }
    
    pub fn clear(&mut self) {
        use std::f32;
        self.data = vec![f32::INFINITY; self.w as usize * self.h as usize].into_boxed_slice()
    }
    
    pub fn draw(&self, window: &mut orbclient::window::Window) {
        for x in 0..self.w {
            for y in 0..self.h {
                let value = self.get(x, y);
                let color = orbclient::Color::rgb((0.5/value * 255.0) as u8,
                                                  (0.5/value * 255.0) as u8,
                                                  (0.5/value * 255.0) as u8);
                window.pixel(x as i32, y as i32, color)
            }
        }
    }
}

/// Draw a triangle, from 9 floats representing vertice coordinates
#[allow(dead_code)]
pub fn triangle_p<T: texture::GetColor>(x1: f32, y1: f32, z1: f32,
                  x2: f32, y2: f32, z2: f32,
                  x3: f32, y3: f32, z3: f32, 
                  color: orbclient::Color, window: &mut orbclient::window::Window,
                  texture: &T, buffer: &mut ZBuffer) {
    use super::{perpective, screen};
    use std::cmp::{min, max};
    
    // Calculate perspective for points.
    let (x1, y1) = perpective(x1, y1, z1);
    let (x2, y2) = perpective(x2, y2, z2);
    let (x3, y3) = perpective(x3, y3, z3);
    
    let scr_width  =  window.width() as i32;
    let scr_height = window.height() as i32;
    
    // Change f32 points into drawable i32, based on screen width, 
    let (x1, y1) = screen(x1, y1, scr_width, scr_height);
    let (x2, y2) = screen(x2, y2, scr_width, scr_height);
    let (x3, y3) = screen(x3, y3, scr_width, scr_height);
    
    let x_low = min(x1, min(x2, x3));
    let x_max = max(x1, max(x2, x3));
    let y_low = min(y1, min(y2, y3));
    let y_max = max(y1, max(y2, y3));
    
    {
        let x1 = x1 as f32;
        let y1 = y1 as f32;
        let x2 = x2 as f32;
        let y2 = y2 as f32;
        let x3 = x3 as f32;
        let y3 = y3 as f32;

        for x in x_low..x_max {
            for y in y_low..y_max {
                    let mut alpha: f32;
                    let mut beta: f32;
                    let mut gamma: f32;
                    
                    let x = x as f32;
                    let y = y as f32;
                    
                    alpha = ((y2 - y3)*(x - x3) + (x3 - x2)*(y - y3)) / ((y2 - y3)*(x1 - x3) + (x3 - x2)*(y1 - y3));
                    beta = ((y3 - y1)*(x - x3) + (x1 - x3)*(y - y3)) / ((y2 - y3)*(x1 - x3) + (x3 - x2)*(y1 - y3));
                    gamma = 1.0 - alpha - beta;
                    
                    if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
                        let z_now = z1 * alpha + z2 * beta + x3 * gamma;                       
                        let z_past = buffer.get(x as u32, y as u32);
                        
                        // If the pixel of the triangle is closer than the one far away(far -> larger value)
                        if z_now < z_past {
                            window.pixel(x as i32, y as i32, texture.get(alpha, beta, gamma));
                            buffer.set(x as u32, y as u32, z_now);
                        }
                    }
            }
        }
    }
  
    
    window.line(x1, y1, x2, y2, color);
    window.line(x2, y2, x3, y3, color);
    window.line(x3, y3, x1, y1, color);
}
    
pub fn triangle_s<T: texture::GetColor>(triangle: &geometry::Triangle,
                  color: orbclient::Color, window: &mut orbclient::window::Window, texture: &T,
                  buffer: &mut ZBuffer) {
    triangle_p(triangle.p1.x, triangle.p1.y, triangle.p1.z,
               triangle.p2.x, triangle.p2.y, triangle.p2.z,
               triangle.p3.x, triangle.p3.y, triangle.p3.z,
               color, window, texture, buffer);                               
}