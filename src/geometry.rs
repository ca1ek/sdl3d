pub struct Point{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle {
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }
} 