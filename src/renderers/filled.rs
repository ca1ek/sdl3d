use orbclient;
use super::super::geometry;
use std;

/// Draw a triangle, from 9 floats representing vertice coordinates
#[allow(dead_code)]
pub fn triangle_p(x1: f32, y1: f32, z1: f32,
                  x2: f32, y2: f32, z2: f32,
                  x3: f32, y3: f32, z3: f32, 
                  color: orbclient::Color, window: &mut orbclient::window::Window) {
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
                        window.pixel(x as i32, y as i32, color);
                    }
            }
        }
    }
  
    
    window.line(x1, y1, x2, y2, color);
    window.line(x2, y2, x3, y3, color);
    window.line(x3, y3, x1, y1, color);
}
    
pub fn triangle_s(triangle: geometry::Triangle,
                  color: orbclient::Color, window: &mut orbclient::window::Window) {
    triangle_p(triangle.p1.x, triangle.p1.y, triangle.p1.z,
               triangle.p2.x, triangle.p2.y, triangle.p2.z,
               triangle.p3.x, triangle.p3.y, triangle.p3.z,
               color, window);                               
}