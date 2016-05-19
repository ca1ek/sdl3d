extern crate orbclient;
extern crate tetrahedrane;

fn main() {
    let mut window = orbclient::window::Window::new_flags(-1, -1, 640, 480, &"dziki wunsz", true).unwrap();
    
    let mut framecounter = 0;
    let mut timecounter = std::time::Duration::new(0,0);
    
    'gameloop: loop {
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
        
        tetrahedrane::renderers::filled::triangle_p(0.0, 0.0, 1.0,
                                                    1.0, 1.0, 1.0,
                                                    1.0, 0.0, 1.0, 
                                                    orbclient::Color::rgb(255,0,255),
                                                    &mut window);
        
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