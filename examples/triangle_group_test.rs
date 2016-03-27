extern crate orbclient;

extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;

fn main() {
    let mut window = start::Window::new(640, 480, "Hello!", 4 as usize);

    let triangle_color = Color::new(200, 200, 200);

    window.window.set(Color::new(20, 40, 60).orb_color());

    let point1 = DepthPoint::new(-0.5, -0.5, 2.0);
    let point2 = DepthPoint::new(-0.5, 0.8, 2.0);
    let point3 = DepthPoint::new(0.2, 0.5, 3.0);

    let mut triangle1 = Triangle::new(point1, point2, point3, 0.0, 0.0, 0.0, triangle_color);

    let point1 = DepthPoint::new(0.5, -0.5, 2.0);
    let point2 = DepthPoint::new(0.5, 0.5, 2.0);
    let point3 = DepthPoint::new(-0.5, -0.5, 2.0);

    let mut triangle2 = Triangle::new(point1, point2, point3, 0.0, 0.0, 0.0, triangle_color);

    let mut group = TriangleGroup::new(vec![triangle1, triangle2]);

    'game_loop: loop {
        window.window.set(Color::new(20, 40, 60).orb_color());

        window.push_group(&group);

        window.normalize_camera();
        window.render();

        window.window.sync();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}