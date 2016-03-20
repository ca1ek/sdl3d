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

    let mut point1 = DepthPoint::new(0.0, 0.0, 2.0);
    let mut point2 = DepthPoint::new(0.5, 0.5, 2.0);

    let mut flat_point1 = point1.flat_point(engine.screen_x, engine.screen_y, 0.0, 0.0, 0.0).make_sdl();
    let flat_point2 = point2.flat_point(engine.screen_x, engine.screen_y, 0.0, 0.0, 0.0).make_sdl();

    println!("{:?}", flat_point1);
    println!("{:?}", flat_point2);

    let mut counter = 0.0;
    'game_loop: loop {
        engine.renderer.set_draw_color(RGB(20, 40, 60));
        engine.renderer.clear();
        engine.renderer.set_draw_color(RGB(200, 200, 200));

        engine.renderer.draw_line(flat_point1, flat_point2);
        engine.renderer.present();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }

    
}