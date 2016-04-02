#[allow(dead_code)]
extern crate rand;
extern crate orbclient;

use super::vid;
use super::start;

pub fn wireframe(id: u16) -> vid::Shader {
    let wireframe_shader = |triangle: &vid::Triangle, window: &mut start::Window, wrapper: &vid::Shader| {
        let flat_1 = triangle.p1.clone().flat_point(window.screen_x, window.screen_y, 
                                            triangle.x + window.camera_x, 
                                            triangle.y + window.camera_y,
                                            triangle.z + window.camera_z);
        let flat_2 = triangle.p2.clone().flat_point(window.screen_x, window.screen_y,
                                            triangle.x + window.camera_x,
                                            triangle.y + window.camera_y,
                                            triangle.z + window.camera_z);
        let flat_3 = triangle.p3.clone().flat_point(window.screen_x, window.screen_y,
                                            triangle.x + window.camera_x,
                                            triangle.y + window.camera_y,
                                            triangle.z + window.camera_z);
        
        window.window.line(flat_1.x, flat_1.y, flat_2.x, flat_2.y, triangle.color.orb_color());
        window.window.line(flat_3.x, flat_3.y, flat_2.x, flat_2.y, triangle.color.orb_color());
        window.window.line(flat_1.x, flat_1.y, flat_3.x, flat_3.y, triangle.color.orb_color());
    };

    vid::Shader::new(id, Box::new(wireframe_shader))
}

pub fn disco_wireframe(id: u16) -> vid::Shader {
    let noise_shader = |triangle: &vid::Triangle, window: &mut start::Window, wrapper: &vid::Shader| {
        let flat_1 = triangle.p1.clone().flat_point(window.screen_x, window.screen_y, 
                                            triangle.x + window.camera_x, 
                                            triangle.y + window.camera_y,
                                            triangle.z + window.camera_z);
        let flat_2 = triangle.p2.clone().flat_point(window.screen_x, window.screen_y,
                                            triangle.x + window.camera_x,
                                            triangle.y + window.camera_y,
                                            triangle.z + window.camera_z);
        let flat_3 = triangle.p3.clone().flat_point(window.screen_x, window.screen_y,
                                            triangle.x + window.camera_x,
                                            triangle.y + window.camera_y,
                                            triangle.z + window.camera_z);
        
        window.window.line(flat_1.x, flat_1.y, flat_2.x, flat_2.y, vid::Color::new(rand::random::<u8>(),
                                                                                   rand::random::<u8>(),
                                                                                   rand::random::<u8>()).orb_color());
        window.window.line(flat_3.x, flat_3.y, flat_2.x, flat_2.y, vid::Color::new(rand::random::<u8>(),
                                                                                   rand::random::<u8>(),
                                                                                   rand::random::<u8>()).orb_color());
        window.window.line(flat_1.x, flat_1.y, flat_3.x, flat_3.y, vid::Color::new(rand::random::<u8>(),
                                                                                   rand::random::<u8>(),
                                                                                   rand::random::<u8>()).orb_color());
    };

    vid::Shader::new(id, Box::new(noise_shader))
}

pub fn filled_triangle_color(id: u16) -> vid::Shader {
    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window, wrapper: &vid::Shader| {
        let p1 = triangle.p1.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x, 
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        let p2 = triangle.p2.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x,
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        let p3 = triangle.p3.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x,
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        struct FloatPoint {
            x: f32,
            y: f32
        }

        let points = [p1, p2, p3];

        let upmost = points.iter().max_by_key(|p| -p.y).unwrap().clone();
        let leftmost = points.iter().max_by_key(|p| -p.x).unwrap().clone();
        let rightmost = points.iter().max_by_key(|p| p.x).unwrap().clone();
        let lowmost = points.iter().max_by_key(|p| p.y).unwrap().clone();

        for px in leftmost.x..rightmost.x {
            for py in upmost.y..lowmost.y {
                let p1 = FloatPoint {x: p1.x as f32, y: p1.y as f32};
                let p2 = FloatPoint {x: p2.x as f32, y: p2.y as f32};
                let p3 = FloatPoint {x: p3.x as f32, y: p3.y as f32};

                let p = FloatPoint {x: px as f32, y: py as f32};

                let alpha = ((p2.y - p3.y)*(p.x - p3.x) + (p3.x - p2.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let beta = ((p3.y - p1.y)*(p.x - p3.x) + (p1.x - p3.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let gamma = 1.0 - alpha - beta;

                if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
                    window.window.pixel(px, py, triangle.color.orb_color());
                }
            }
        }
    };

    vid::Shader::new(id, Box::new(rasterize_shader))
}

pub fn filled_b_w_noise(id: u16) -> vid::Shader {
    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window, wrapper: &vid::Shader| {
        let p1 = triangle.p1.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x, 
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        let p2 = triangle.p2.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x,
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        let p3 = triangle.p3.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x,
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        struct FloatPoint {
            x: f32,
            y: f32
        }

        let points = [p1, p2, p3];

        let upmost = points.iter().max_by_key(|p| -p.y).unwrap().clone();
        let leftmost = points.iter().max_by_key(|p| -p.x).unwrap().clone();
        let rightmost = points.iter().max_by_key(|p| p.x).unwrap().clone();
        let lowmost = points.iter().max_by_key(|p| p.y).unwrap().clone();

        for px in leftmost.x..rightmost.x {
            for py in upmost.y..lowmost.y {
                let p1 = FloatPoint {x: p1.x as f32, y: p1.y as f32};
                let p2 = FloatPoint {x: p2.x as f32, y: p2.y as f32};
                let p3 = FloatPoint {x: p3.x as f32, y: p3.y as f32};

                let p = FloatPoint {x: px as f32, y: py as f32};

                let alpha = ((p2.y - p3.y)*(p.x - p3.x) + (p3.x - p2.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let beta = ((p3.y - p1.y)*(p.x - p3.x) + (p1.x - p3.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let gamma = 1.0 - alpha - beta;

                let random = rand::random::<u8>();

                if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
                    window.window.pixel(px, py, vid::Color::new(random, random, random).orb_color());
                }
            }
        }
    };

    vid::Shader::new(id, Box::new(rasterize_shader))
}

pub fn filled_texture_naive(id: u16, texture_path: &str) -> vid::Shader {
    let img = orbclient::BmpFile::from_path(texture_path);

    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window, wrapper: &vid::Shader| {
        use std::ops::Deref;

        let p1 = triangle.p1.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x, 
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        let p2 = triangle.p2.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x,
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        let p3 = triangle.p3.clone().flat_point(window.screen_x, window.screen_y,
                                                triangle.x + window.camera_x,
                                                triangle.y + window.camera_y,
                                                triangle.z + window.camera_z);

        struct FloatPoint {
            x: f32,
            y: f32
        }

        let img_w = wrapper.image_data.width();
        let img_h = wrapper.image_data.height();

        let img_slice = wrapper.image_data.deref();

        let points = [p1, p2, p3];

        let upmost = points.iter().max_by_key(|p| -p.y).unwrap().clone();
        let leftmost = points.iter().max_by_key(|p| -p.x).unwrap().clone();
        let rightmost = points.iter().max_by_key(|p| p.x).unwrap().clone();
        let lowmost = points.iter().max_by_key(|p| p.y).unwrap().clone();

        for px in leftmost.x..rightmost.x {
            for py in upmost.y..lowmost.y {
                let p1 = FloatPoint {x: p1.x as f32, y: p1.y as f32};
                let p2 = FloatPoint {x: p2.x as f32, y: p2.y as f32};
                let p3 = FloatPoint {x: p3.x as f32, y: p3.y as f32};

                let p = FloatPoint {x: px as f32, y: py as f32};

                let alpha = ((p2.y - p3.y)*(p.x - p3.x) + (p3.x - p2.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let beta = ((p3.y - p1.y)*(p.x - p3.x) + (p1.x - p3.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let gamma = 1.0 - alpha - beta;

                let random = rand::random::<u8>();

                if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
                    window.window.pixel(px, py, img_slice[(px + py*(img_w as i32)) as usize]);
                }
            }
        }
    };

    let mut shader = vid::Shader::new(id, Box::new(rasterize_shader));

    shader.image_data = img;

    shader
}
