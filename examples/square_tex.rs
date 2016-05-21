extern crate orbclient;
extern crate orbimage;
extern crate tetrahedrane;

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
                                      
    let img = orbimage::Image::from_path(&"bmp/redox.png").unwrap();
    
    let texture1 = tetrahedrane::texture::FloatTexture::from_image(&img, (0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
    let texture2 = tetrahedrane::texture::FloatTexture::from_image(&img, (1.0, 1.0), (0.0, 1.0), (0.0, 0.0));
    
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
        
        tetrahedrane::renderers::textured::triangle_s(&triangle1, 
                                                    orbclient::Color::rgb(255,0,255),
                                                    &mut window, &texture1);
                                                    
        tetrahedrane::renderers::textured::triangle_s(&triangle2, 
                                                    orbclient::Color::rgb(255,0,255),
                                                    &mut window, &texture2);
                                                    
        /*let (n_x, n_y) = rotate(triangle1.p1.x, triangle1.p1.y, 0.0, 0.0, 0.01);
        
        println!("{} {}", n_x, n_y);
        
        triangle1.p1.x = n_x;
        triangle1.p1.y = n_y;*/
        //triangle1.rotate_x_y(0.0, 0.0, 0.01);
        //triangle2.rotate_x_y(0.0, 0.0, 0.01);
        //triangle1.rotate_x_z(0.0, 1.0, 0.01);
        //triangle2.rotate_x_z(0.0, 1.0, 0.01);
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