extern crate tetrahedrane;

use tetrahedrane::render::*;
use tetrahedrane::geometry::*;

fn main() {
    let mut window = tetrahedrane::init::Window::new(20, 20, 640, 480, &"Hello World");
    let mut framebuffer = Framebuffer::new(640, 480, window);

	let texture = Texture::load(&"bmp/crate.bmp");

    let mut counter = 0;

    let mut p2z = 1.5;

    let flat_point = FlatPoint {x: 1.0, y: 0.0};

    println!("{:?}", flat_point.screen_point(480, 640));

    loop {
        let mut triangle = Triangle::new(Point::new(-1.0, 1.0, 3.0),
                                     Point::new(1.0/p2z, 1.0/p2z, p2z),
                                     Point::new(0.0, 1.0, 3.0),
                                     &texture);

        let mut triangles: Vec<&Triangle> = vec![&triangle];

        framebuffer.render_3d_triangles(&triangles, 480, 640);
        //framebuffer.draw_cached_zbuf();
        //window.apply_z_buf(&framebuffer); // now responsibility of framebuffer
        framebuffer.sync();
        framebuffer.clear();

        triangle.get_normal();

        p2z += 0.1;
        std::thread::sleep(std::time::Duration::from_millis(33));

        counter += 1;
        if counter > 60 {
            break;
        }
    }
}
