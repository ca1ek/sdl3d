extern crate rand;
extern crate orbclient;

use super::vid;
use super::start;
use super::texture;

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

/*
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
                    window.window.pixel(px, py, img_slice[(px + py *(img_w as i32)) as usize]);
                }
            }
        }
    };

    let mut shader = vid::Shader::new(id, Box::new(rasterize_shader));

    shader.image_data = img;

    shader
}*/

pub fn filled_texture(id: u16, texture: texture::UVTexture, uv_p1x: f32, uv_p1y: f32, uv_p2x: f32, uv_p2y: f32, uv_p3x: f32, uv_p3y: f32) -> vid::Shader {
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

        let img_w = wrapper.texture.bmp.width();
        let img_h = wrapper.texture.bmp.height();

        let img_slice = wrapper.texture.bmp.deref();

        let points = [p1, p2, p3];

        let upmost = points.iter().max_by_key(|p| -p.y).unwrap().clone();
        let leftmost = points.iter().max_by_key(|p| -p.x).unwrap().clone();
        let rightmost = points.iter().max_by_key(|p| p.x).unwrap().clone();
        let lowmost = points.iter().max_by_key(|p| p.y).unwrap().clone();

        let w = rightmost.x - leftmost.x;
        let h = lowmost.y - upmost.y;

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
                    let uv_p1x = wrapper.flags[0];
                    let uv_p1y = wrapper.flags[1];
                    let uv_p2x = wrapper.flags[2];
                    let uv_p2y = wrapper.flags[3];
                    let uv_p3x = wrapper.flags[4];
                    let uv_p3y = wrapper.flags[5];

                    let uv_p1 = FloatPoint {x: (p1.x - w as f32) / w as f32, y: (p1.y - h as f32) / h as f32};
                    let uv_p2 = FloatPoint {x: (p2.x - w as f32) / w as f32, y: (p2.y - h as f32) / h as f32};
                    let uv_p3 = FloatPoint {x: (p3.x - w as f32) / w as f32, y: (p3.y - h as f32) / h as f32};

                    //println!("{:?}", uv_p1.x);

                    let uv_p = FloatPoint {x: (alpha * uv_p1x + beta * uv_p2x + gamma * uv_p3x) as f32, y: (alpha * uv_p1y + beta * uv_p2y + gamma * uv_p3y) as f32};

                    //println!("{:?}", uv_p.x);

                    let texture = &wrapper.texture;

                    window.window.pixel(px, py, texture.get_by_uv(uv_p.x, uv_p.y));
                }
            }
        }
    };

    let mut shader = vid::Shader::new(id, Box::new(rasterize_shader));

    shader.texture = texture;

    shader.flags[0] = uv_p1x;
    shader.flags[1] = uv_p1y;
    shader.flags[2] = uv_p2x;
    shader.flags[3] = uv_p2y;
    shader.flags[4] = uv_p3x;
    shader.flags[5] = uv_p3y;

    shader
}

/*pub fn filled_gradient_depth(id: u16, texture_path: &str) -> vid::Shader {
    let img = orbclient::BmpFile::from_path(texture_path);

    let rasterize_shader = |triangle: &vid::Triangle, window: &mut start::Window, wrapper: &vid::Shader| {
        use std::ops::Deref;

        #[derive(Clone, Copy)]
        struct ZPoint {
            x: i32,
            y: i32,
            z: f32,
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

        let p1 = ZPoint {x: p1.x, y: p1.y, z: triangle.p1.z};
        let p2 = ZPoint {x: p2.x, y: p2.y, z: triangle.p2.z};
        let p3 = ZPoint {x: p3.x, y: p3.y, z: triangle.p3.z};

        struct FloatPoint {
            x: f32,
            y: f32,
            z: f32,
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
                let p1 = FloatPoint {x: p1.x as f32, y: p1.y as f32, z: p1.z};
                let p2 = FloatPoint {x: p2.x as f32, y: p2.y as f32, z: p2.z};
                let p3 = FloatPoint {x: p3.x as f32, y: p3.y as f32, z: p3.z};

                let p = FloatPoint {x: px as f32, y: py as f32, z: 0.0};

                let alpha = ((p2.y - p3.y)*(p.x - p3.x) + (p3.x - p2.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let beta = ((p3.y - p1.y)*(p.x - p3.x) + (p1.x - p3.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
                let gamma = 1.0 - alpha - beta;

                let random = rand::random::<u8>();

                if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
                    let p1_dist = ((px as f32 - p1.x)*(px as f32 - p1.x) + (py as f32 - p1.y)*(py as f32 - p1.y)).sqrt();
                    let p2_dist = ((px as f32 - p2.x)*(px as f32 - p2.x) + (py as f32 - p2.y)*(py as f32 - p2.y)).sqrt();
                    let p3_dist = ((px as f32 - p3.x)*(px as f32 - p3.x) + (py as f32 - p3.y)*(py as f32 - p3.y)).sqrt();

                    let norm_p1 = p1_dist / (p1_dist + p2_dist + p3_dist);
                    let norm_p2 = p2_dist / (p1_dist + p2_dist + p3_dist);
                    let norm_p3 = p3_dist / (p1_dist + p2_dist + p3_dist);

                    let norm_p1z = p1.z / (p1.z + p2.z + p3.z);
                    let norm_p2z = p2.z / (p1.z + p2.z + p3.z);
                    let norm_p3z = p3.z / (p1.z + p2.z + p3.z);          

                    let z = (norm_p1 * norm_p1z + norm_p2 * norm_p2z + norm_p3 * norm_p3z);

                    window.window.pixel(px, py, vid::Color::new((z * 255.0) as u8, (z * 255.0) as u8, (z * 255.0) as u8).orb_color());
                }
            }
        }
    };

    let mut shader = vid::Shader::new(id, Box::new(rasterize_shader));

    shader.image_data = img;

    shader
}*/

pub fn filled_triangle_gradient(id: u16) -> vid::Shader {
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
                    let p1_dist = ((px as f32 - p1.x)*(px as f32 - p1.x) + (py as f32 - p1.y)*(py as f32 - p1.y)).sqrt();
                    let p2_dist = ((px as f32 - p2.x)*(px as f32 - p2.x) + (py as f32 - p2.y)*(py as f32 - p2.y)).sqrt();
                    let p3_dist = ((px as f32 - p3.x)*(px as f32 - p3.x) + (py as f32 - p3.y)*(py as f32 - p3.y)).sqrt();

                    window.window.pixel(px, py, vid::Color::new(p1_dist as u8, p2_dist as u8, p3_dist as u8).orb_color());
                }
            }
        }
    };

    vid::Shader::new(id, Box::new(rasterize_shader))
}
