extern crate orbclient;
extern crate sinulation;

use super::start;

#[cfg(target_os = "redox")] // if os is redox use these trig functions instead of the ones from standard lib
use sinulation::Trig;

pub struct Shader {
    pub id: u16,
    pub shader: Box<Fn(&Triangle, &mut start::Window, &Shader)>,
    pub image_data: orbclient::BmpFile,
}

impl Shader {
    pub fn new(id: u16, shader: Box<Fn(&Triangle, &mut start::Window, &Shader)>) -> Shader {
        Shader {
            id: id,
            shader: shader,
            image_data: orbclient::BmpFile::default(),
        }
    }

    pub fn null() -> Shader {
        Shader {
            id: 0,
            shader: Box::new(|triangle: &Triangle, window: &mut start::Window, wrapper: &Shader| {}),
            image_data: orbclient::BmpFile::default(),
        }
    }

    /*pub fn apply(self, triangle: &Triangle) {
        (self.shader)(triangle);
    }*/
}

/// Color struct. Stores colors in 8-bit RGB.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    /// Converts tetrahedrane colors to ones accepted by orbclient library.
    pub fn orb_color(&self) -> orbclient::color::Color {
        orbclient::color::Color::rgb(self.r, self.g, self.b)
    }
}

/// 2D point. Coordinates are screen pixels.
#[derive(Clone, Copy, Debug)]
pub struct FlatPoint {
    pub x: i32,
    pub y: i32,
}

/// 3D point. Coordinates are floating point, similar to OpenGL coordinates. 
///
/// 0.0 is center, -1.0 is left, 1.0 is right etc.
///
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
    /// Creates a new 3D point.
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

    /// Converts into 2D point, with perspective.
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

    /// Applies camera rotations from variables `x_y`, `x_z` and `y_z`
    pub fn apply_camera_rotations(&mut self, engine: &start::Window) {
        #[cfg(not(target_os = "redox"))]
        use std::f32::consts::PI;

        #[cfg(target_os = "redox")]
        const PI: f32 = 3.141592653589793;
        
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

    pub fn camera_rotate_x_y(&mut self, engine: &start::Window, angle: f32) {
        #[cfg(not(target_os = "redox"))]
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
    
    pub fn camera_rotate_x_z(&mut self, engine: &start::Window, angle: f32) {
        #[cfg(not(target_os = "redox"))]
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

    pub fn camera_rotate_y_z(&mut self, engine: &start::Window, angle: f32) {
        #[cfg(not(target_os = "redox"))]
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

    /// Rotates the point around provided coordinates by the angle.
    pub fn coord_rotate_x_y(&mut self, x: f32, y: f32, angle: f32) {
        #[cfg(not(target_os = "redox"))]
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
    
    /// Rotates the point around provided coordinates by the angle.
    pub fn coord_rotate_x_z(&mut self, x: f32, z: f32, angle: f32) {
        #[cfg(not(target_os = "redox"))]
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

    /// Rotates the point around provided coordinates by the angle.
    pub fn coord_rotate_y_z(&mut self, y: f32, z: f32, angle: f32) {
        #[cfg(not(target_os = "redox"))]
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

/// Triangle. `p1`, `p2`, `p3` are it's vertexes.
#[derive(Clone, Copy)]
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

    pub color: Color,

    pub shader_ids: [u16; 8],
}

impl Triangle {
    /// Creates a new triangle
    pub fn new(p1: DepthPoint, p2: DepthPoint, p3: DepthPoint, x: f32, y: f32, z: f32, color: Color) -> Triangle {
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
            
            color: color,

            shader_ids: [0; 8],
        }
    }

    /// Applies camera rotations from variables `x_y`, `x_z` and `y_z`
    pub fn apply_camera_rotations(&mut self, engine: &start::Window) {
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

    /// Rotates the point around provided coordinates by the angle.
    pub fn coord_rotate_x_y(&mut self, x: f32, y: f32, angle: f32) {
        self.p1.coord_rotate_x_y(x, y, angle);
        self.p2.coord_rotate_x_y(x, y, angle);
        self.p3.coord_rotate_x_y(x, y, angle);
    }

    /// Rotates the point around provided coordinates by the angle.
    pub fn coord_rotate_x_z(&mut self, x: f32, z: f32, angle: f32) {
        self.p1.coord_rotate_x_z(x, z, angle);
        self.p2.coord_rotate_x_z(x, z, angle);
        self.p3.coord_rotate_x_z(x, z, angle);
    }

    /// Rotates the point around provided coordinates by the angle.
    pub fn coord_rotate_y_z(&mut self, y: f32, z: f32, angle: f32) {
        self.p1.coord_rotate_y_z(y, z, angle);
        self.p2.coord_rotate_y_z(y, z, angle);
        self.p3.coord_rotate_y_z(y, z, angle);
    }
}

/// A group of triangles.
//#[derive(Clone)]
pub struct TriangleGroup {
    pub triangles: Vec<Triangle>,
}

impl TriangleGroup {
    /// Create a new group of triangles.
    pub fn new(triangles: Vec<Triangle>) -> TriangleGroup {
        TriangleGroup {
            triangles: triangles
        }
    }
}
