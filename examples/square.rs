extern crate orbclient;
extern crate tetrahedrane;

fn rotate(x: f32, y: f32, around_x: f32, around_y: f32, angle: f32) -> (f32, f32) {
    use std::f32;
    
    let s = f32::sin(angle);
    let c = f32::cos(angle);
    
    let x = x - around_x;
    let y = y - around_y;
    
    let x = x * c - y * s;
    let y = x * s + y * c;
    
    (x + around_x, y + around_y)
}

fn main() {
    use tetrahedrane::geometry::{Triangle, Point};
    let mut window = orbclient::window::Window::new_flags(-1, -1, 640, 480, &"dziki wunsz", true).unwrap();
    
    let mut framecounter = 0;
    let mut timecounter = std::time::Duration::new(0,0);
    
    let mut triangle1 = Triangle::new(Point::new(-0.5, -0.5, 1.0),
                                      Point::new( 0.5, -0.5, 1.0),
                                      Point::new( 0.5,  0.5, 1.0));
                                      
    let mut triangle2 = Triangle::new(Point::new( 0.5,  0.5, 1.0),
                                      Point::new(-0.5,  0.5, 1.0),
                                      Point::new(-0.5, -0.5, 1.0));
    
    'gameloop: loop {
        use tetrahedrane::util::Rotation;
        
        let begin = std::time::Instant::now();
        
        for event in window.events() {
            use orbclient::event::EventOption;
            let ev_opt = event.to_option();
            
            match ev_opt {
                EventOption::Quit(..) => break 'gameloop,
                _ => {}
            }
        }
        
        window.clear();
        
        tetrahedrane::renderers::filled::triangle_s(&triangle1, 
                                                    orbclient::Color::rgb(255,0,255),
                                                    &mut window);
                                                    
        tetrahedrane::renderers::filled::triangle_s(&triangle2, 
                                                    orbclient::Color::rgb(255,0,255),
                                                    &mut window);
                                                    
        /*let (n_x, n_y) = rotate(triangle1.p1.x, triangle1.p1.y, 0.0, 0.0, 0.01);
        
        println!("{} {}", n_x, n_y);
        
        triangle1.p1.x = n_x;
        triangle1.p1.y = n_y;*/
        triangle1.rotate_x_y(0.0, 0.0, 0.01);
        triangle2.rotate_x_y(0.0, 0.0, 0.01);
        triangle1.rotate_x_z(0.0, 1.0, 0.01);
        triangle2.rotate_x_z(0.0, 1.0, 0.01);
        triangle1.rotate_y_z(0.0, 1.0, 0.02);
        triangle2.rotate_y_z(0.0, 1.0, 0.02);
        
        
        window.sync();
        
        let end = std::time::Instant::now();
        
        framecounter += 1;
        timecounter += end.duration_since(begin);
        
        if timecounter > std::time::Duration::new(1, 0) {
            timecounter = std::time::Duration::new(0, 0);
            println!("FPS: {}", framecounter);
            framecounter = 0;
        }
    }
}