extern crate sdl2;
extern crate sdl2_sys;
extern crate libc;

mod start;
mod vid;

use vid::*;

fn main() {
    let mut rend_contx = start::bootstrap(1280, 720, "Hello world!");
    let (mut renderer, mut pump) = rend_contx;

    unsafe {sdl2_sys::mouse::SDL_SetRelativeMouseMode(1);}

    let screen_w = 1280/2;
    let screen_h = 720/2;

    let mut camera_x = 0.0;
    let mut camera_y = 0.0;
    let mut camera_z = 3.0; 

    let mut camera_x_z = 0.0;
    let mut camera_y_z = 0.0;
    let mut camera_x_y = 0.0;

    let mut cubes = vec![Cube::gen_new(0.0, 0.0, 4.0, 0.5, 0.5, 0.5),
                         Cube::gen_new(0.0, 0.0, 1.0, 2.5, 0.5, 0.5),
                         Cube::gen_new(0.0, 0.0, 1.0, 0.5, 0.5, 0.5),
                         /*Cube::gen_new(1.0, 1.0, 1.0, 0.5, 0.5, 0.5),*/];

    let mut lines = Lines::new(vec![
                                [DepthPoint::new(-0.5, -0.5, 0.6), DepthPoint::new(-0.5, 0.5, 0.6)],
                                [DepthPoint::new(-0.5, -0.5, 0.6), DepthPoint::new(0.0, -0.5, 0.6)],
                                [DepthPoint::new(-0.5, 0.0, 0.6), DepthPoint::new(0.0, 0.0, 0.6)],
                                [DepthPoint::new(0.0, 0.0, 0.6), DepthPoint::new(0.0, -0.5, 0.6)],
                               ]);

    let mut triangle = Triangle::new(DepthPoint::new(0.0, -1.0, 1.0), DepthPoint::new(-1.0, 1.0, 1.0), DepthPoint::new(1.0, 1.0, 1.0));

    let horizon = DepthPoint::new(0.0, 0.5, 1.0);

    'game_loop: loop {
        use sdl2::pixels::Color::RGB;

        std::thread::sleep(std::time::Duration::from_millis(33));
        
        for event in pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit {..} => {break 'game_loop;},
                KeyDown {keycode, ..} => {
                    match keycode {
                        Some(Up) => {
                            let z = camera_z;
                            camera_z = z - 0.05;
                        },
                        Some(Down) => {
                            let z = camera_z;
                            camera_z = z + 0.05;
                        },
                        Some(Left) => {
                            let x = camera_x;
                            camera_x = x + 0.05;
                        },
                        Some(Right) => {
                            let x = camera_x;
                            camera_x = x - 0.05;
                        },
                        Some(RCtrl) => {
                            let y = camera_y;
                            camera_y = y + 0.05;
                        },
                        Some(RShift) => {
                            let y = camera_y;
                            camera_y = y - 0.05;
                        },
                        Some(Q) => {
                            camera_x_y += 0.1;
                        },
                        Some(E) => {
                            camera_x_y -= 0.1;
                        },
                        Some(Escape) => {
                            break 'game_loop;
                        }
                        _ => {println!("{:?}", keycode);}
                    }
                },
                MouseMotion {xrel, yrel, ..} => {
                    camera_x_z = (xrel as f64)/30.0;
                    //camera_y_z = (yrel as f64)/30.0;
                }
                _ => {}
            }
        }

        renderer.set_draw_color(RGB(20, 40, 60));
        renderer.clear();
        renderer.set_draw_color(RGB(200, 200, 200));

        // UNCOMMENT TO TAKE A LOOK AT LINES, IF I REMEMBER CORRECTLY IT WILL MAKE A SHAPE OF LETTER 'P'
        //lines.flat(screen_w, screen_h, &mut renderer,
        //           camera_x, camera_y, camera_z,
        //          camera_x_y, camera_x_z, camera_y_z);


        /* // UNCOMMENT TO TAKE A LOOK  AT DRAWING CUBES
        // should show four cubes, you can define more yourself in cubes vector.
        // KNOWN BUG: IF A POINT IS OFF SCREEN THEN THE WHOLE LINE ISNT DRAWN AND IT STARTS GOING TO WRONG POINT
        for cube in &mut cubes {
            use std::f64;

            cube.flat(screen_w, screen_h, &mut renderer, 
                      camera_x, camera_y, camera_z, 
                      camera_x_y, camera_x_z, camera_y_z);
            
        }// end for cube in cubes */
        
        triangle.flat(screen_w, screen_h, &mut renderer,
                      camera_x, camera_y, camera_z,
                      camera_x_y, camera_x_z, camera_y_z);
        triangle.fill_bottom_flat(screen_w, screen_h);

        // Reset relative mouse move back to 0 as everything was already moved
        camera_x_z = 0.0;
        camera_y_z = 0.0; 
        camera_x_y = 0.0;
        

        renderer.present(); 
    }
}
