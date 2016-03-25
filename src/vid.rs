extern crate sdl2;

use super::start;

#[derive(Clone, Copy, Debug)]
pub struct FlatPoint {
    pub x: i32,
    pub y: i32,
}

impl FlatPoint {
    pub fn make_sdl(&self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DepthPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub x_y: f32,
    pub x_z: f32,
    pub y_z: f32,

    last_x_y: f32,
    last_x_z: f32,
    last_y_z: f32,
}

impl DepthPoint {
    pub fn new(x: f32, y: f32, z: f32) -> DepthPoint {
        DepthPoint {
            x: x, 
            y: y,
            z: z,

            x_y: 0.0,
            x_z: 0.0,
            y_z: 0.0,
            
            last_x_y: 0.0,
            last_x_z: 0.0,
            last_y_z: 0.0,
        }
    }

    pub fn flat_point(&mut self, engine_scr_x: u32, engine_scr_y: u32, offset_x: f32, offset_y: f32, offset_z: f32) -> FlatPoint { 
        if self.z > -0.01 && self.z < 0.0 {
            self.z = 0.001
        }

        else if self.z < 0.1 { // Prevents division by nearly 0, that cause integer overflow/underflow
            self.z = 0.11;
        }

        FlatPoint {
            x: ((engine_scr_x as f32 * (self.x + offset_x) as f32/(self.z + offset_z)) + engine_scr_x as f32 / 2.0) as i32, 
            y: ((engine_scr_x as f32 * (self.y + offset_y) as f32/(self.z + offset_z)) + engine_scr_y as f32 / 2.0) as i32,
        }
    }

    pub fn apply_camera_rotations(&mut self, engine: &start::Engine) {
        use std::f32::consts::PI;

        let x_y = self.x_y;
        let x_z = self.x_z;
        let y_z = self.y_z;

        let last_x_y = self.last_x_y;
        let last_x_z = self.last_x_z;
        let last_y_z = self.last_y_z;

        self.camera_rotate_x_y(&engine, x_y - last_x_y);
        self.camera_rotate_x_z(&engine, x_z - last_x_z);
        self.camera_rotate_y_z(&engine, y_z - last_y_z);

        self.last_x_y = x_y;
        self.last_x_z = x_z;
        self.last_y_z = y_z;

        //normalize rotations
        if self.x_z > (PI * 2.0) {
            self.x_z -= (PI * 2.0);
        }

        if self.x_y > (PI * 2.0) {
            self.x_y -= (PI * 2.0);
        }

        if self.y_z > (PI * 2.0) {
            self.y_z -= (PI * 2.0);
        }        
    }

    pub fn camera_rotate_x_y(&mut self, engine: &start::Engine, angle: f32) {
        use std::f32;
        let s = f32::sin(angle);
        let c = f32::cos(angle);

        self.x -= engine.camera_x;
        self.y -= engine.camera_y;

        let new_x = self.x * c - self.y * s;
        let new_y = self.x * s + self.y * c;

        self.x = new_x + engine.camera_x;
        self.y = new_y + engine.camera_y;
    }
    
    pub fn camera_rotate_x_z(&mut self, engine: &start::Engine, angle: f32) {
        use std::f32;
        let s = f32::sin(angle);
        let c = f32::cos(angle);

        self.x -= engine.camera_x;
        self.z -= engine.camera_z;

        let new_x = self.x * c - self.z * s;
        let new_z = self.x * s + self.z * c;

        self.x = new_x + engine.camera_x;
        self.z = new_z + engine.camera_z;
    }

    pub fn camera_rotate_y_z(&mut self, engine: &start::Engine, angle: f32) {
        use std::f32;
        let s = f32::sin(angle);
        let c = f32::cos(angle);

        self.y -= engine.camera_y;
        self.z -= engine.camera_z;

        let new_y = self.y * c - self.z * s;
        let new_z = self.y * s + self.z * c;

        self.y = new_y + engine.camera_y;
        self.z = new_z + engine.camera_z;
    }  

    pub fn coord_rotate_x_y(&mut self, x: f32, y: f32, angle: f32) {
        use std::f32;
        let s = f32::sin(angle);
        let c = f32::cos(angle);

        self.x -= x;
        self.y -= y;

        let new_x = self.x * c - self.y * s;
        let new_y = self.x * s + self.y * c;

        self.x = new_x + x;
        self.y = new_y + y;
    }
    
    pub fn coord_rotate_x_z(&mut self, x: f32, z: f32, angle: f32) {
        use std::f32;
        let s = f32::sin(angle);
        let c = f32::cos(angle);

        self.x -= x;
        self.z -= z;

        let new_x = self.x * c - self.z * s;
        let new_z = self.x * s + self.z * c;

        self.x = new_x + x;
        self.z = new_z + z;
    }

    pub fn coord_rotate_y_z(&mut self, y: f32, z: f32, angle: f32) {
        use std::f32;
        let s = f32::sin(angle);
        let c = f32::cos(angle);

        self.y -= y;
        self.z -= z;

        let new_y = self.y * c - self.z * s;
        let new_z = self.y * s + self.z * c;

        self.y = new_y + y;
        self.z = new_z + z;
    }  
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: DepthPoint,
    pub p2: DepthPoint,
    pub p3: DepthPoint,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub x_y: f32,
    pub x_z: f32,
    pub y_z: f32,
}

impl Triangle {
    pub fn new(p1: DepthPoint, p2: DepthPoint, p3: DepthPoint, x: f32, y: f32, z: f32) -> Triangle {
        Triangle {
            p1: p1,
            p2: p2, 
            p3: p3,

            x: x,
            y: y, 
            z: z,

            x_y: 0.0,
            x_z: 0.0,
            y_z: 0.0,
            
        }
    }

    pub fn apply_camera_rotations(&mut self, engine: &start::Engine) {
        self.p1.x_y += self.x_y;
        self.p1.x_z += self.x_z;
        self.p1.y_z += self.y_z;

        self.p2.x_y += self.x_y;
        self.p2.x_z += self.x_z;
        self.p2.y_z += self.y_z;

        self.p3.x_y += self.x_y;
        self.p3.x_z += self.x_z;
        self.p3.y_z += self.y_z;

        self.p1.apply_camera_rotations(&engine);
        self.p2.apply_camera_rotations(&engine);
        self.p3.apply_camera_rotations(&engine);

        self.p1.x_y -= self.x_y;
        self.p1.x_z -= self.x_z;
        self.p1.y_z -= self.y_z;

        self.p2.x_y -= self.x_y;
        self.p2.x_z -= self.x_z;
        self.p2.y_z -= self.y_z;

        self.p3.x_y -= self.x_y;
        self.p3.x_z -= self.x_z;
        self.p3.y_z -= self.y_z;
    }
}

pub struct TriangleGroup {
    pub triangles: Vec<Triangle>,
}

impl TriangleGroup {
    pub fn new(triangles: Vec<Triangle>) -> TriangleGroup {
        TriangleGroup {
            triangles: triangles
        }
    }
}
