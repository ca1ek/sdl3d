extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;
use tetrahedrane::shaders;
use tetrahedrane::texture;

fn main() {
    let mut window = start::Window::new(640, 480, "Hello World!", 1 as usize);

    let mut shaders: Vec<Shader> = Vec::new();

    let triangle = Triangle::new(DepthPoint::new(0.0, -0.5, 3.0),  
                                 DepthPoint::new(0.5, 0.5, 3.0), 
                                 DepthPoint::new(-0.5, 0.5, 3.0), 
                                 0.0, 0.0, 0.0,
                                 Color::new(200, 200, 200));

    let mut triangle_group = TriangleGroup::cube(-0.5, -0.5, 3.0, 1.0, 1.0, 1.0);

    //let mut triangle_group = TriangleGroup::new(vec![triangle]);

    let texture = texture::UVTexture::path(&"bmp/crate.bmp");
    shaders.push(shaders::filled_texture(1, texture, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0));

    let texture = texture::UVTexture::path(&"bmp/crate.bmp");
    shaders.push(shaders::filled_texture(2, texture, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0));
    //shaders.push(shaders::filled_triangle_color(1));
    //shaders.push(shaders::wireframe(1));

    //triangle_group.triangles[0].shader_ids[0] = 1;

    loop {
        window.window.set(Color::new(20, 40, 60).orb_color());
        window.window.set(Color::new(20, 40, 60).orb_color());

        let triangles = &mut triangle_group.triangles[..];

        triangles.sort

        for face in triangles {
            face.coord_rotate_x_y(0.0, 0.0, 0.01);
            face.coord_rotate_x_z(0.0, 0.0, 0.02);
            face.coord_rotate_y_z(0.0, 0.0, 0.03);
        }

        /*window.render_addon_shader(triangle_group.triangles[0], &shaders, [1, 0, 0, 0, 0, 0, 0, 0]); 
        window.render_addon_shader(triangle_group.triangles[1], &shaders, [2, 0, 0, 0, 0, 0, 0, 0]);
        window.render_addon_shader(triangle_group.triangles[2], &shaders, [1, 0, 0, 0, 0, 0, 0, 0]);
        window.render_addon_shader(triangle_group.triangles[3], &shaders, [2, 0, 0, 0, 0, 0, 0, 0]);*/

        window.window.sync();


        //std::thread::sleep(std::time::Duration::from_millis(33));
    }
}