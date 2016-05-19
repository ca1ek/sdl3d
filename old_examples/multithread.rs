extern crate tetrahedrane;
extern crate orbclient;
extern crate rand;

use tetrahedrane::vid::*;
use tetrahedrane::start;
use tetrahedrane::shaders;
use tetrahedrane::texture;

fn main() {
    use std::sync::mpsc;

    let (t1_group_in, t1_group_out): (mpsc::Sender<TriangleGroup>, mpsc::Receiver<TriangleGroup>) = mpsc::channel();

    let (t1_event_in, t1_event_out): (mpsc::Sender<orbclient::window::EventIter>, mpsc::Receiver<orbclient::window::EventIter>) = mpsc::channel();

    let mut triangle_group = TriangleGroup::square(-0.5, -0.5, 3.0, 1.0, 1.0);

    std::thread::spawn(move || {
        let mut window = start::Window::new(640, 480, "Hello World!", 1 as usize);
        let mut shaders: Vec<Shader> = Vec::new();

        let texture = texture::UVTexture::path(&"bmp/crate.bmp");
        shaders.push(shaders::filled_texture(1, texture, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0));

        let texture = texture::UVTexture::path(&"bmp/crate.bmp");
        shaders.push(shaders::filled_texture(2, texture, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0));

        loop {
            let triangle_group = t1_group_out.recv().unwrap();
            window.window.set(Color::new(20, 40, 60).orb_color());
            window.window.set(Color::new(20, 40, 60).orb_color());

            window.render_addon_shader(triangle_group.triangles[0], &shaders, [1, 0, 0, 0, 0, 0, 0, 0]);
            window.render_addon_shader(triangle_group.triangles[1], &shaders, [2, 0, 0, 0, 0, 0, 0, 0]);

            window.window.sync();
            t1_event_in.send(window.window.events());

            let wait = (rand::random::<u8>() / 10) as u64;

            std::thread::sleep(std::time::Duration::from_millis((rand::random::<u8>() / 10) as u64));
            println!("{:?}", wait);
        }
    });


    'game_loop: loop {
        for triangle in &mut triangle_group.triangles {
            triangle.coord_rotate_x_y(0.5, 0.5, 0.01);
            triangle.coord_rotate_x_z(0.5, 0.0, 0.02);
            triangle.coord_rotate_y_z(0.5, 0.0, 0.03);
        }

        t1_group_in.send(triangle_group.clone());

        std::thread::sleep(std::time::Duration::from_millis(16));

        let mut events = t1_event_out.try_recv();

        if events.is_ok() {
            for event in events.unwrap().next() {
                if event.code == 3 {
                    break 'game_loop;
                }
            }
        }
    }
}
