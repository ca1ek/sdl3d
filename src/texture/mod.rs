use orbclient;
use orbimage;

fn get_by_coord(texture: &orbimage::Image, x: u32, y: u32) -> orbclient::color::Color {
    let data  =  texture.data();
    let width = texture.width();
    
    if (x + y * width) as usize > data.len() {
        return orbclient::color::Color::rgb(0, 0, 0);
    }
    data[(x + y * width) as usize]
}

fn get_by_barycentric(texture: &orbimage::Image, a: (u32, u32), b: (u32, u32), g: (u32, u32),
                                                 alpha: f32,    beta: f32,     gamma: f32,  ) -> orbclient::color::Color {
    let x = (a.0 as f32 * alpha + b.0 as f32 * beta + g.0 as f32 * gamma) as u32;
    let y = (a.1 as f32 * alpha + b.1 as f32 * beta + g.1 as f32 * gamma) as u32;
    
    get_by_coord(texture, x, y)
}

fn get_by_barycentric_f(texture: &orbimage::Image, a: (f32, f32), b: (f32, f32), g: (f32, f32),
                                                 alpha: f32,    beta: f32,     gamma: f32,  ) -> orbclient::color::Color {
                                                     
    let width = texture.width();
    let height = texture.height();
    let a = ((a.0 * width as f32) as u32, (a.1 * height as f32) as u32);
    let b = ((b.0 * width as f32) as u32, (b.1 * height as f32) as u32);
    let g = ((g.0 * width as f32) as u32, (g.1 * height as f32) as u32);
    
    get_by_barycentric(texture, a, b, g, alpha, beta, gamma)
                                            
}

pub trait GetColor {
    fn get(&self, alpha: f32, beta: f32, gamma: f32) -> orbclient::color::Color;
}

pub struct Texture<'a> {
    img: &'a orbimage::Image,
    map_alpha: (u32, u32),
    map_beta: (u32, u32),
    map_gamma: (u32, u32),
}

impl<'a> Texture<'a> {
    pub fn from_image(img: &'a orbimage::Image, map_alpha: (u32, u32), map_beta: (u32, u32), map_gamma: (u32, u32)) -> Texture<'a> {
        Texture {
            img: img,
            map_alpha: map_alpha,
            map_beta: map_beta,
            map_gamma: map_gamma,
        }
    }
    
    pub fn get(&self, alpha: f32, beta: f32, gamma: f32) -> orbclient::color::Color {
        get_by_barycentric(&self.img, self.map_alpha, self.map_beta, self.map_gamma, alpha, beta, gamma)
    }
}

impl<'a> GetColor for Texture<'a> {
    fn get(&self, alpha: f32, beta: f32, gamma: f32) -> orbclient::color::Color {
        get_by_barycentric(&self.img, self.map_alpha, self.map_beta, self.map_gamma, alpha, beta, gamma)
    }
}

pub struct FloatTexture<'a> {
    img: &'a orbimage::Image,
    map_alpha: (f32, f32),
    map_beta: (f32, f32),
    map_gamma: (f32, f32),
}

impl<'a> FloatTexture<'a> {
    pub fn from_image(img: &'a orbimage::Image, map_alpha: (f32, f32), map_beta: (f32, f32), map_gamma: (f32, f32)) -> FloatTexture<'a> {
        FloatTexture {
            img: img,
            map_alpha: map_alpha,
            map_beta: map_beta,
            map_gamma: map_gamma,
        }
    }
}

impl<'a> GetColor for FloatTexture<'a> {
    fn get(&self, alpha: f32, beta: f32, gamma: f32) -> orbclient::color::Color {
        get_by_barycentric_f(&self.img, self.map_alpha, self.map_beta, self.map_gamma, alpha, beta, gamma)
    }
}