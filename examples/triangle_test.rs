extern crate orbclient;

extern crate sdl3d;

use sdl3d::vid::*;
use sdl3d::start;

fn main() {

    let mut engine = start::Engine::new(1280, 720, "Hello!", 1 as usize);

    engine.window.set(Color::new(20, 40, 60).orb_color());

    let point1 = DepthPoint::new(0.0, -0.5, 2.0);
    let point2 = DepthPoint::new(0.5, 0.5, 2.0);
    let point3 = DepthPoint::new(-0.5, 0.5, 2.0);

    let triangle = Triangle::new(point1, point2, point3, 0.0, 0.0, 0.0, Color::new(200, 200, 200));

    'game_loop: loop {
        engine.window.set(Color::new(20, 40, 60).orb_color());


        engine.camera_z += 0.01;

        engine.render_queue.push(triangle);

        engine.render();

        engine.window.sync();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}