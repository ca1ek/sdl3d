extern crate orbclient;

extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;

fn main() {

    let mut window = start::Window::new(1280, 720, "Hello!", 1 as usize);

    window.window.set(Color::new(20, 40, 60).orb_color());

    let point1 = DepthPoint::new(0.0, -0.5, 0.0);
    let point2 = DepthPoint::new(0.5, 0.5, 0.0);
    let point3 = DepthPoint::new(-0.5, 0.5, 0.0);

    let mut triangle = Triangle::new(point1, point2, point3, 0.0, 0.0, 2.0, Color::new(200, 200, 200));

    'game_loop: loop {
        window.window.set(Color::new(20, 40, 60).orb_color());

        let mut events = window.window.events();
        for event in events.next() {
            if event.code == 3 {
                break 'game_loop;
            }
        }
        //window.camera_z += 0.01;

        triangle.coord_rotate_x_y(0.0, 0.0, 0.01);
        triangle.coord_rotate_x_z(0.0, 0.0, 0.02);
        triangle.coord_rotate_y_z(0.0, 0.0, 0.03);

        window.render_queue.push(triangle);

        window.render();

        window.window.sync();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}