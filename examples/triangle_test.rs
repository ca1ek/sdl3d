extern crate sdl2;
extern crate sdl2_sys;
extern crate libc;

extern crate sdl3d;

use sdl3d::vid::*;
use sdl3d::start;

fn main() {
    use sdl2::pixels::Color::RGB;

    let mut engine = start::Engine::new(1280, 720, "Hello!".to_string(), 1 as usize);

    engine.renderer.set_draw_color(RGB(20, 40, 60));
    engine.renderer.clear();
    engine.renderer.set_draw_color(RGB(200, 200, 200));

    let point1 = DepthPoint::new(0.0, -0.5, 2.0);
    let point2 = DepthPoint::new(0.5, 0.5, 2.0);
    let point3 = DepthPoint::new(-0.5, 0.5, 2.0);

    let triangle = Triangle::new(point1, point2, point3, 0.0, 0.0, 0.0);

    'game_loop: loop {
        engine.camera_z += 0.01;
        engine.renderer.set_draw_color(RGB(20, 40, 60));
        engine.renderer.clear();
        engine.renderer.set_draw_color(RGB(200, 200, 200));

        engine.render_queue.push(triangle);

        engine.render();

        engine.renderer.present();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}