extern crate orbclient;

extern crate tetrahedrane;

use tetrahedrane::vid::*;
use tetrahedrane::start;

fn main() {
    let mut window = start::Window::new(640, 480, "Hello!", 4 as usize);

    let triangle_color = Color::new(200, 200, 200);

    window.window.set(Color::new(20, 40, 60).orb_color());

    let point1 = DepthPoint::new(1.0, 1.0, 1.0);
    let point2 = DepthPoint::new(1.0, -1.0, -1.0);
    let point3 = DepthPoint::new(-1.0, 1.0, -1.0);

    let mut triangle1 = Triangle::new(point1, point2, point3, 0.0, 0.0, 4.0, triangle_color);

    let point1 = DepthPoint::new(1.0, -1.0, -1.0);
    let point2 = DepthPoint::new(-1.0, 1.0, -1.0);
    let point3 = DepthPoint::new(-1.0, -1.0, 1.0);

    let mut triangle2 = Triangle::new(point1, point2, point3, 0.0, 0.0, 4.0, triangle_color);

    let point1 = DepthPoint::new(-1.0, 1.0, -1.0);
    let point2 = DepthPoint::new(-1.0, -1.0, 1.0);
    let point3 = DepthPoint::new(1.0, 1.0, 1.0);

    let mut triangle3 = Triangle::new(point1, point2, point3, 0.0, 0.0, 4.0, triangle_color);

    'game_loop: loop {
        window.window.set(Color::new(20, 40, 60).orb_color());

        let mut events = window.window.events();
        for event in events.next() {
            if event.code == 3 {
                break 'game_loop;
            }
        }

        triangle1.coord_rotate_x_y(0.0, 0.0, 0.02);
        triangle1.coord_rotate_x_z(0.0, 0.0, 0.03);
        triangle1.coord_rotate_y_z(0.0, 0.0, 0.01);

        triangle2.coord_rotate_x_y(0.0, 0.0, 0.02);
        triangle2.coord_rotate_x_z(0.0, 0.0, 0.03);
        triangle2.coord_rotate_y_z(0.0, 0.0, 0.01);

        triangle3.coord_rotate_x_y(0.0, 0.0, 0.02);
        triangle3.coord_rotate_x_z(0.0, 0.0, 0.03);
        triangle3.coord_rotate_y_z(0.0, 0.0, 0.01);

        window.camera_z = 4.0;

        let mut group = TriangleGroup::new(vec![triangle1.clone(), triangle2.clone(), triangle3.clone()]);

        window.push_group(&group);

        window.normalize_camera();
        window.render();

        window.window.sync();
        //std::thread::sleep(std::time::Duration::from_millis(33));
    }
}