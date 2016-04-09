pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn make_2d(&self) -> FlatPoint {
        FlatPoint {
            x: self.x / self.z,
            y: self.y / self.z,
        }
    }
}

pub struct FlatPoint {
    x: f32,
    y: f32,
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    uv_p1: FlatPoint,
    uv_p2: FlatPoint,
    uv_p3: FlatPoint,
}
