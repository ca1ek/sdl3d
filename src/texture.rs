extern crate orbclient;

#[derive(Clone, Copy, Debug)]
pub struct UVPoint {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct UVData {
    p1: UVPoint,
    p2: UVPoint,
    p3: UVPoint,
}

pub struct UVTexture {
    pub bmp: orbclient::BmpFile, 
    pub p1: UVPoint,
    pub p2: UVPoint,
    pub p3: UVPoint,
}

impl UVTexture {
    pub fn default() -> UVTexture {
        UVTexture {
            bmp: orbclient::BmpFile::default(), 
            p1: UVPoint {x: 0.0, y: 0.0},
            p2: UVPoint {x: 0.0, y: 1.0},
            p3: UVPoint {x: 1.0, y: 0.0},
        }
    }

    pub fn path(path: &str) -> UVTexture {
        UVTexture {
            bmp: orbclient::BmpFile::from_path(path), 
            p1: UVPoint {x: 0.0, y: 0.0},
            p2: UVPoint {x: 0.0, y: 1.0},
            p3: UVPoint {x: 1.0, y: 0.0},
        } 
    }

    pub fn get_by_uv(&self, x: f32, y: f32) -> orbclient::Color {
        use std::ops::Deref;

        let px = (x * self.bmp.width() as f32) as i32;
        let py = (y * self.bmp.height() as f32) as i32;

        let img_w = self.bmp.width();

        let img_slice = self.bmp.deref();

        if (px + py *(img_w as i32)) as usize > 260000 {
            orbclient::Color::rgb(255,255,255)
        } else {
            img_slice[(px + py *(img_w as i32)) as usize]
        }
    }
}