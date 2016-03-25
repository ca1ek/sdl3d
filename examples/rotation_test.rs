extern crate sdl2;
extern crate sdl2_sys;
extern crate libc;

extern crate sdl3d;

use sdl3d::vid::*;
use sdl3d::start;

fn main() {
    use sdl2::pixels::Color::RGB;

    let mut engine = start::Engine::new(1280, 720, "Hello!".to_string());

    engine.renderer.set_draw_color(RGB(20, 40, 60));
    engine.renderer.clear();
    engine.renderer.set_draw_color(RGB(200, 200, 200));

    let point1 = DepthPoint::new(0.0, -0.5, 2.0);
    let point2 = DepthPoint::new(0.5, 0.5, 2.0);
    let point3 = DepthPoint::new(-0.5, 0.5, 2.0);

    let mut triangle = Triangle::new(point1, point2, point3, 0.0, 0.0, 0.0);

    'game_loop: loop {
        engine.renderer.set_draw_color(RGB(20, 40, 60));
        engine.renderer.clear();
        engine.renderer.set_draw_color(RGB(200, 200, 200));

        for event in engine.event_pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit {..} => {break 'game_loop;},
                MouseMotion {xrel, yrel, ..} => {
                    engine.camera_x_z += xrel as f32 / 20.0;
                },

                _ => {}
            }
        }

        triangle.x_z = engine.camera_x_z;

        triangle.apply_camera_rotations(&engine);

        engine.render_queue.push(triangle);

        engine.normalize_camera();
        engine.render();

        engine.renderer.present();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}