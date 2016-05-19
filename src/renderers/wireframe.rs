use orbclient;
use super::super::geometry;

/// Draw a triangle, from 9 floats representing vertice coordinates
#[allow(dead_code)]
pub fn triangle_p(x1: f32, y1: f32, z1: f32,
                  x2: f32, y2: f32, z2: f32,
                  x3: f32, y3: f32, z3: f32, 
                  color: orbclient::Color, window: &mut orbclient::window::Window) {
    use super::{perpective, screen};
    
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
    
    window.line(x1, y1, x2, y2, color);
    window.line(x2, y2, x3, y3, color);
    window.line(x3, y3, x1, y1, color);
}
    
pub fn triangle_s(triangle: &geometry::Triangle,
                  color: orbclient::Color, window: &mut orbclient::window::Window) {
    triangle_p(triangle.p1.x, triangle.p1.y, triangle.p1.z,
               triangle.p2.x, triangle.p2.y, triangle.p2.z,
               triangle.p3.x, triangle.p3.y, triangle.p3.z,
               color, window);                               
}