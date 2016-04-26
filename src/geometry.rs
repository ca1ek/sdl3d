pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn make_2d(&self) -> FlatPoint {
        FlatPoint {
            x: self.x / self.z,
            y: self.y / self.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FlatPoint {
    pub x: f32,
    pub y: f32,
}

impl FlatPoint {
    pub fn screen_point(&self, screen_h: i32) -> (i32, i32) {
        (screen_h/2 + (self.x * (screen_h/2) as f32) as i32, screen_h/2 + (self.y * (screen_h/2) as f32) as i32)
    }
}

pub struct Triangle<'a> {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub uv_p1: FlatPoint,
    pub uv_p2: FlatPoint,
    pub uv_p3: FlatPoint,
    texture: &'a super::render::Texture,
}

impl<'a> Triangle<'a> {
    pub fn new(p1: Point, p2: Point, p3: Point, texture: &'a super::render::Texture) -> Triangle<'a> {
        Triangle {
            p1: p1,
            p2: p2,
            p3: p3,
            uv_p1: FlatPoint {x: 0.0, y: 0.0},
            uv_p2: FlatPoint {x: 1.0, y: 0.0},
            uv_p3: FlatPoint {x: 0.0, y: 1.0},
            texture: texture,
        }
    }

    pub fn make_2d(&self) -> FlatTriangle<'a> {
        FlatTriangle {
            p1: self.p1.make_2d(),
            p2: self.p2.make_2d(),
            p3: self.p3.make_2d(),
            uv_p1: self.uv_p1.clone(),
            uv_p2: self.uv_p2.clone(),
            uv_p3: self.uv_p3.clone(),
            texture: self.texture,
        }
    }
}

#[derive(Debug)]
pub struct FlatTriangle<'a> {
    pub p1: FlatPoint,
    pub p2: FlatPoint,
    pub p3: FlatPoint,
    pub uv_p1: FlatPoint,
    pub uv_p2: FlatPoint,
    pub uv_p3: FlatPoint,
    texture: &'a super::render::Texture,
}

impl<'a> FlatTriangle<'a> {
    pub fn get_barycentric(&self, x: u32, y: u32, screen_h: i32) -> (f32, f32, f32) {
        #[derive(Debug)]
        struct ScreenPoint {
            x: f32,
            y: f32,
        }

        let (p1x, p1y) = self.p1.screen_point(screen_h);
        let (p2x, p2y) = self.p2.screen_point(screen_h);
        let (p3x, p3y) = self.p3.screen_point(screen_h);

        let p1 = ScreenPoint {x: p1x as f32, y: p1y as f32};
        let p2 = ScreenPoint {x: p2x as f32, y: p2y as f32};
        let p3 = ScreenPoint {x: p3x as f32, y: p3y as f32};

        //println!("{:?} {:?} {:?}", p1, p2, p3);

        let p = ScreenPoint {x: x as f32, y: y as f32};

        //println!("{:?}", ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y)));

        let alpha = ((p2.y - p3.y)*(p.x - p3.x) + (p3.x - p2.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
        let beta = ((p3.y - p1.y)*(p.x - p3.x) + (p1.x - p3.x)*(p.y - p3.y)) / ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));
        let gamma = 1.0 - alpha - beta;

        (alpha, beta, gamma)
    }

    pub fn inside(&self, x: u32, y: u32, screen_h: i32) -> bool {
        let (alpha, beta, gamma) = self.get_barycentric(x, y, screen_h);
        //println!("{:?} {:?} {:?}", alpha, beta, gamma);
        if alpha > 0.0 && beta > 0.0 && gamma > 0.0 {
            true
        } else {
            false
        }
    }
}
