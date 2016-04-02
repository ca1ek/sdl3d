extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;
use tetrahedrane::shaders;

fn main() {
    // Make a new window
    let mut window = start::Window::new(640, 480, "Hello World!", 1 as usize);

    let mut shaders: Vec<Shader> = Vec::new();

    // Create a triangle
    let mut triangle = Triangle::new(DepthPoint::new(0.0, -0.5, 3.0),  
                                 DepthPoint::new(0.5, 0.5, 3.0), 
                                 DepthPoint::new(-0.5, 0.5, 3.0), 
                                 0.0, 0.0, 0.0,
                                 Color::new(200, 200, 200));

    // Add a shader
    shaders.push(shaders::filled_triangle_gradient(1));

    // Apply shader to the triangle
    triangle.shader_ids[0] = 1;

    loop {
        window.window.set(Color::new(20, 40, 60).orb_color());
        window.window.set(Color::new(20, 40, 60).orb_color());

        triangle.coord_rotate_x_y(0.0, 0.0, 0.01);
        triangle.coord_rotate_x_z(0.0, 3.0, 0.02);
        triangle.coord_rotate_y_z(0.0, 3.0, 0.05);

        // Render the triangle with shaders.
        window.render(triangle, &shaders); 

        window.window.sync();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}