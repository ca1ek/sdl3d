extern crate orbclient;

extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;

fn main() {

    let mut window = start::Window::new(640, 480, "Hello!", 1 as usize);

    window.window.set(Color::new(20, 40, 60).orb_color());

    let mut point1 = DepthPoint::new(0.0, -0.5, 3.0);
    let mut point2 = DepthPoint::new(0.5, 0.7, 3.0);
    let mut point3 = DepthPoint::new(-0.5, 0.5, 3.0);

    let triangle = Triangle::new(point1, point2, point3, 0.0, 0.0, 0.0, Color::new(200, 200, 200));
    
    let mut counter = 0;

    'game_loop: loop {
        window.window.set(Color::new(20, 40, 60).orb_color());

        //window.camera_z += 0.01;

        counter += 1;

        window.render_queue.push(triangle);

        window.render();

        window.window.line(0, counter, 10 + counter, 10 - counter , Color::new(200, 40, 60).orb_color());

        window.window.sync();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}