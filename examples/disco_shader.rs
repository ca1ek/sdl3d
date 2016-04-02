extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;
use tetrahedrane::shaders;

fn main() {
    let mut window = start::Window::new(640, 480, "Hello World!", 1 as usize);

    let mut shaders: Vec<Shader> = Vec::new();

    let mut triangle = Triangle::new(DepthPoint::new(0.0, -0.5, 3.0),  
                                 DepthPoint::new(0.5, 0.5, 3.0), 
                                 DepthPoint::new(-0.5, 0.5, 3.0), 
                                 0.0, 0.0, 0.0,
                                 Color::new(200, 200, 200));

    shaders.push(shaders::disco_wireframe(1));

    triangle.shader_ids[0] = 1;

    loop {
        window.window.set(Color::new(20, 40, 60).orb_color());
        window.window.set(Color::new(20, 40, 60).orb_color());

        triangle.coord_rotate_x_y(0.0, 0.0, 0.01);

        window.render(triangle, &shaders); 

        window.window.sync();

        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}