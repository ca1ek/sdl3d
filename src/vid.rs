extern crate sdl2;

#[derive(Debug)]
pub struct DepthPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_z: f64,
    pub angle_view: f64,
}

impl DepthPoint {
    pub fn new(x: f64, y: f64, z: f64) -> DepthPoint {
        DepthPoint {
            x: x, 
            y: y,
            z: z,
            x_z: 0.0,
            angle_view: 0.0,
        }
    }

    pub fn sdl_point(&self) -> sdl2::rect::Point {
        sdl2::rect::Point::new((self.x as f64/self.z) as i32, (self.y as f64/self.z) as i32)
    }

    pub fn perspect_point(&mut self, w: i32, h: i32) -> sdl2::rect::Point { 
        if self.z > -0.01 && self.z < 0.0 {
            self.z = 0.001
        }

        else if self.z < 0.1 { // Prevents division by nearly 0, that cause integer overflow/underflow
            self.z = 0.11;
        }

        sdl2::rect::Point::new(
            ((w as f64 * self.x as f64/self.z) + w as f64) as i32, 
            ((w as f64 * self.y as f64/self.z) + h as f64) as i32)
    }

    pub fn set_x (&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y (&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_z (&mut self, z: f64) {
        self.z = z;
    }

    pub fn rotate_x_y(&mut self, cx: f64, cy: f64, angle: f64) {
        use std::f64;
        let s = f64::sin(angle);
        let c = f64::cos(angle);

        self.x -= cx;
        self.y -= cy;

        let new_x = self.x * c - self.y * s;
        let new_y = self.x * s + self.y * c;

        self.x = new_x + cx;
        self.y = new_y + cy;
    }

    pub fn rotate_x_z(&mut self, cx: f64, cz: f64, angle: f64) {
        use std::f64;
        let s = f64::sin(angle);
        let c = f64::cos(angle);

        self.x -= cx;
        self.z -= cz;

        let new_x = self.x * c - self.z * s;
        let new_z = self.x * s + self.z * c;

        self.x = new_x + cx;
        self.z = new_z + cz;
        self.x_z += angle;

        let angle_view = (f64::atan2(cx - self.x, cz - self.z) * 180.0 / f64::consts::PI);

        self.angle_view = angle_view;

    }

    pub fn rotate_y_z(&mut self, cy: f64, cz: f64, angle: f64) {
        use std::f64;
        let s = f64::sin(angle);
        let c = f64::cos(angle);

        self.y -= cy;
        self.z -= cz;

        let new_y = self.y * c - self.z * s;
        let new_z = self.y * s + self.z * c;

        self.y = new_y + cy;
        self.z = new_z + cz;
    }

    pub fn clone(&self) -> DepthPoint {
        DepthPoint {
            x: self.x, 
            y: self.y,
            z: self.z,
            x_z: self.x_z,
            angle_view: self.angle_view,
        }
    }
}
#[derive(Debug)]
pub struct Square {
    pub points: Vec<DepthPoint>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub angle_view: f64,
}

impl Square {
    pub fn new(p1: DepthPoint, p2: DepthPoint, p3: DepthPoint, p4: DepthPoint) -> Square {
        Square {
            points: vec![p1, p2, p3, p4],
            x: 0.0,
            y: 0.0,
            z: 0.0,
            angle_view: 0.0,
        }
    }

    pub fn flat(&mut self, w: i32, h: i32, renderer: &mut sdl2::render::Renderer, cx: f64, cy: f64, cz: f64) {
        let mut return_buffer = Vec::<sdl2::rect::Point>::new();
        for point in &mut self.points { 
            use std::f64;
            let point_x = point.x;
            let point_y = point.y;
            let point_z = point.z;

            point.set_x(point_x+self.x);
            point.set_y(point_y+self.y);
            point.set_z(point_z+self.z);

            let pers_point = point.perspect_point(w, h); 


            if !(point.angle_view < 100.0 && point.angle_view > 0.0
            || point.angle_view < 0.0 && point.angle_view > -100.0)  {
                return_buffer.push(pers_point);
            }
            /* CAUSES A COOL BUG WHEN UNCOMMENTED! I RECOMMEND TRYING IT OUT.
            * Bug itself happens because the points position doesn't get reset when i just jump out to the next point in the loop.
            else {
                continue;
            }
            */
            
            
            point.set_x(point_x);
            point.set_y(point_y);
            point.set_z(point_z);
        }
        
        let point_x = self.points[0].x;
        let point_y = self.points[0].y; 
        let point_z = self.points[0].z;

        self.points[0].set_x(point_x+self.x);
        self.points[0].set_y(point_y+self.y);
        self.points[0].set_z(point_z+self.z);
        
        return_buffer.push(self.points[0].perspect_point(w, h));
        renderer.draw_lines(&return_buffer);

        self.points[0].set_x(point_x);
        self.points[0].set_y(point_y);
        self.points[0].set_z(point_z);
    }

    pub fn set_x (&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y (&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_z (&mut self, z: f64) {
        self.z = z;
    }
}

#[derive(Debug)]
pub struct Cube {
    pub faces: Vec<Square>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Cube {
    pub fn new(p1: Square, p2: Square, p3: Square, p4: Square) -> Cube {
        Cube {
            faces: vec![p1, p2, p3, p4],
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn gen_new(x: f64, y: f64, z:f64, x_s: f64, y_s: f64, z_s:f64) -> Cube {
        let face1 = Square::new(DepthPoint::new(x_s, y_s, z_s), 
                                DepthPoint::new(-x_s, y_s, z_s), 
                                DepthPoint::new(-x_s, -y_s, z_s),
                                DepthPoint::new(x_s, -y_s, z_s));

        let face2 = Square::new(DepthPoint::new(x_s, y_s, -z_s), 
                                DepthPoint::new(x_s, -y_s, -z_s), 
                                DepthPoint::new(x_s, -y_s, z_s),
                                DepthPoint::new(x_s, y_s, z_s));

        let face3 = Square::new(DepthPoint::new(x_s, y_s, -z_s), 
                                DepthPoint::new(-x_s, y_s, -z_s), 
                                DepthPoint::new(-x_s, -y_s, -z_s),
                                DepthPoint::new(x_s, -y_s, -z_s));

        let face4 = Square::new(DepthPoint::new(-x_s, -y_s, -z_s), 
                                DepthPoint::new(-x_s, y_s, -z_s), 
                                DepthPoint::new(-x_s, y_s, z_s),
                                DepthPoint::new(-x_s, -y_s, z_s));

        Cube {
            faces: vec![face1, face2, face3, face4],
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn flat(&mut self, w: i32, h: i32, renderer: &mut sdl2::render::Renderer, cx: f64, cy: f64, cz: f64, cxy: f64, cxz: f64, cyz: f64) {
        for face in &mut self.faces {
            let self_x = self.x;
            let self_y = self.y;
            let self_z = self.z;

            self.x += cx;
            self.y += cy;
            self.z += cz;

            for point in &mut face.points {
                    //let point_x = point.x;
                    //let point_y = point.y;
                    //let point_z = point.z;
                    point.rotate_y_z((-cy + -self_y), (-cz + -self_z), cyz);
                    point.rotate_x_z((-cx + -self_x), (-cz + -self_z), cxz);
                    point.rotate_x_y((-cx + -self_x), (-cy + -self_y), cxy);
            }

            let face_x = face.x;
            let face_y = face.y;
            let face_z = face.z;

            face.set_x(face_x+self.x);
            face.set_y(face_y+self.y);
            face.set_z(face_z+self.z);

            let flat = face.flat(w, h, renderer, cx, cy, cz);

            face.set_x(face_x);
            face.set_y(face_y);
            face.set_z(face_z);

            self.x = self_x;
            self.y = self_y;
            self.z = self_z;
        }
    }
}

pub struct Lines {
    pub lines: Vec<[DepthPoint; 2]>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Lines {
    pub fn new(line_vec: Vec<[DepthPoint; 2]>) -> Lines {
        Lines {
            lines: line_vec,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn flat(&mut self, w: i32, h: i32, renderer: &mut sdl2::render::Renderer,
                           cx: f64, cy: f64, cz: f64, 
                           cxy: f64, cxz: f64, cyz: f64) {
        for line in &mut self.lines {
            // Apply rotations
            let self_x = self.x;
            let self_y = self.y;
            let self_z = self.z;

            self.x += cx;
            self.y += cy;
            self.z += cz;

            line[0].rotate_y_z((-cy + -self_y), (-cz + -self_z), cyz);
            line[0].rotate_x_z((-cx + -self_x), (-cz + -self_z), cxz);
            line[0].rotate_x_y((-cx + -self_x), (-cy + -self_y), cxy);

            line[1].rotate_y_z((-cy + -self_y), (-cz + -self_z), cyz);
            line[1].rotate_x_z((-cx + -self_x), (-cz + -self_z), cxz);
            line[1].rotate_x_y((-cx + -self_x), (-cy + -self_y), cxy);

            if !(line[0].angle_view < 100.0 && line[0].angle_view > 0.0
            || line[0].angle_view < 0.0 && line[0].angle_view > -100.0) {
                // Grab all the positions because Rust doesn't allow for a method 
                // to take its instances class variable as an argument
                let mut line_begin_x = line[0].x;
                let mut line_begin_y = line[0].y;
                let mut line_begin_z = line[0].z;

                let mut line_end_x = line[1].x;
                let mut line_end_y = line[1].y;
                let mut line_end_z = line[1].z;

                line[0].x = line_begin_x + self.x;
                line[0].y = line_begin_y + self.y;
                line[0].z = line_begin_z + self.z;

                line[1].x = line_end_x + self.x;
                line[1].y = line_end_y + self.y;
                line[1].z = line_end_z + self.z;

                // Generate 2d lines. 

                let line_begin = line[0].perspect_point(w, h); 
                let line_end = line[1].perspect_point(w, h);
                renderer.draw_lines(&[line_begin, line_end]);

                line[0].x = line_begin_x;
                line[0].y = line_begin_y;
                line[0].z = line_begin_z;

                line[1].x = line_end_x;
                line[1].y = line_end_y;
                line[1].z = line_end_z;     
            }
            
            // Set points' positions back to the ones they had before.
            self.x = self_x;
            self.y = self_y;
            self.z = self_z;
        }
    }
}

pub struct Triangle {
    pub points: [DepthPoint; 3],
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Triangle {
    pub fn new(p1: DepthPoint, p2: DepthPoint, p3: DepthPoint) -> Triangle {
        Triangle {
            points: [p1, p2, p3],
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn flat(&mut self, w: i32, h: i32, renderer: &mut sdl2::render::Renderer,
                           cx: f64, cy: f64, cz: f64, 
                           cxy: f64, cxz: f64, cyz: f64) {

        //MOST PROBABLY USELESS MEMORY REALLOCATIONS, BUT SUCH IS LIFE.
        let mut lines = Lines::new(vec![[self.points[0].clone(), self.points[1].clone()],
                                        [self.points[1].clone(), self.points[2].clone()],
                                        [self.points[2].clone(), self.points[0].clone()],]);

        lines.flat(w, h, renderer,
                   cx, cy, cz,
                   cxy, cxz, cyz);

        self.points = [lines.lines[0][0].clone(), lines.lines[1][0].clone(), lines.lines[2][0].clone()];
    }

    pub fn fill_bottom_flat(&self, w: i32, h: i32) {
        let mut largest_y = -2147483648; // used value from someone gave on irc because std::i32::MIN wasnt feeling like working at the moment 
        let mut largest_index = 0 as usize;

        let flat_p1 = self.points[0].clone().perspect_point(w, h);
        let flat_p2 = self.points[1].clone().perspect_point(w, h);
        let flat_p3 = self.points[2].clone().perspect_point(w, h);

        let mut top: sdl2::rect::Point;
        let mut left: sdl2::rect::Point;
        let mut right: sdl2::rect::Point;
        // find top, left, and right.
        {
            let points = [flat_p1, flat_p2, flat_p3];
            for i in 0..3 {
                if points[i].y() > largest_y {
                    largest_index = i;
                    largest_y = points[i].y();
                }
            }

            top = points[largest_index];
            if largest_index == 0 {
                if points[1].x() > points[2].x() {
                    left = points[1];
                    right = points[2];
                }
                else {
                    left = points[2];
                    right = points[1];
                }
            }
            else if largest_index == 1 {
                if points[0].x() > points[2].x() {
                    left = points[0];
                    right = points[2];
                }
                else {
                    left = points[2];
                    right = points[0];
                }
            }
            else {
                if points[0].x() > points[1].x() {
                    left = points[0];
                    right = points[1];
                }
                else {
                    left = points[1];
                    right = points[0];
                }
            }
        }
        
        if (left.y() - top.y()) != 0 && (right.y() - top.y()) != 0 {
            let left_slope = (left.x() - top.x()) / (left.y() - top.y());
            let right_slope = (right.x() - top.x()) / (right.y() - top.y());
            println!("{:?}", left_slope);
        }
    }
}