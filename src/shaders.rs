#[allow(dead_code)]
extern crate rand;

use super::vid;
use super::start;

pub fn wireframe(id: u16) -> vid::Shader {
    let wireframe_shader = |triangle: &vid::Triangle, window: &mut start::Window| {
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

    vid::Shader {id: id, shader: Box::new(wireframe_shader)}
}

pub fn disco_wireframe(id: u16) -> vid::Shader {
    let noise_shader = |triangle: &vid::Triangle, window: &mut start::Window| {
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

    vid::Shader {id: id, shader: Box::new(noise_shader)}
}

/// GARBAGE, WILL REMOVE SOON
pub fn garbage_filled(id: u16) -> vid::Shader {
    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window| {

        fn bottom_flat(top: vid::FlatPoint, left: vid::FlatPoint, right: vid::FlatPoint, triangle: &vid::Triangle, window: &mut start::Window) {
            if (left.y - top.y) != 0 && (right.y - top.y) != 0 {
                let left_slope = -(left.x - top.x) as f64 / (left.y - top.y) as f64;
                let right_slope = -(right.x - top.x) as f64 / (right.y - top.y) as f64;

                for i in 0..left.y - top.y {
                    window.window.line(right.x + (right_slope * i as f64) as i32, right.y - i,
                                       left.x + (left_slope * i as f64) as i32, left.y - i, triangle.color.orb_color());
                }
            }
        }

        fn top_flat(left: vid::FlatPoint, right: vid::FlatPoint, top: vid::FlatPoint, triangle: &vid::Triangle, window: &mut start::Window) {
            if (left.y - top.y) != 0 && (right.y - top.y) != 0 {
                let left_slope = -(left.x - top.x) as f64 / (left.y - top.y) as f64;
                let right_slope = -(right.x - top.x) as f64 / (right.y - top.y) as f64;

                for i in 0..top.y - left.y {
                    window.window.line(right.x + (right_slope * -i as f64) as i32, right.y + i,
                                       left.x + (left_slope * -i as f64) as i32, left.y + i, triangle.color.orb_color());
                }
            }
        }

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

        let points = [p1, p2, p3];

        let top = points.iter().max_by_key(|p| -p.y).unwrap().clone();
        let left = points.iter().max_by_key(|p| -p.x).unwrap().clone();
        let right = points.iter().max_by_key(|p| p.x).unwrap().clone();

        /*if top.y == left.y { // top flat
            println!("{:?} {:?}", top.y, left.y);
            let top = points.iter().max_by_key(|p| p.y).unwrap().clone();
            top_flat(left, right, top, &triangle, window);
            println!("top_flat");
        } else if left.y == right.y { // bottom flat
            bottom_flat(top, left, right, &triangle, window);
            println!("bot_flat");
        } else {*/
            let left_and_right = [left.clone(), right.clone()];
            let middle = left_and_right.iter().max_by_key(|p| -p.y).unwrap().clone();
            let low = left_and_right.iter().max_by_key(|p| p.y).unwrap().clone();

            let magic = (top.x as f32 + ((middle.y - top.y) as f32 / (low.y - top.y) as f32) * (low.x - top.x) as f32) as i32;

            let new_point = vid::FlatPoint { x: magic , y: middle.y };

            bottom_flat(top, middle, new_point, &triangle, window);
            top_flat(middle, new_point, low,  &triangle, window);
            println!("comm");
        //}

    };

    vid::Shader {id: id, shader: Box::new(rasterize_shader)}
}

pub fn filled_triangle_color(id: u16) -> vid::Shader {
    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window| {
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

    vid::Shader {id: id, shader: Box::new(rasterize_shader)}
}

pub fn filled_b_w_noise(id: u16) -> vid::Shader {
    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window| {
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

    vid::Shader {id: id, shader: Box::new(rasterize_shader)}
}
