extern crate tetrahedrane;

fn main() {
    let mut window = tetrahedrane::init::Window::new(20, 20, 640, 480, &"Hello World");
    let mut framebuffer = tetrahedrane::render::Framebuffer::new(640, 480);

    let red = tetrahedrane::render::Color::new(255, 0, 0);

    loop {
        for x in 0..100 {
            for y in 0..100 {
                framebuffer.set_pixel(x, y, &red);
            }
        }

        window.apply_buf(&framebuffer);

        window.sync();

        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
