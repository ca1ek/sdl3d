extern crate orbclient;
extern crate tetrahedrane;

fn main() {
    let mut window = orbclient::window::Window::new_flags(-1, -1, 640, 480, &"dziki wunsz", true).unwrap();
    
    'gameloop: loop {
        for event in window.events() {
            use orbclient::event::EventOption;
            let ev_opt = event.to_option();
            
            match ev_opt {
                EventOption::Quit(..) => break 'gameloop,
                _ => {}
            }
        }
        
        window.clear();
        
        tetrahedrane::renderers::wireframe::triangle_p(0.0, 0.0, 1.0,
                                                    1.0, 1.0, 1.0,
                                                    1.0, 0.0, 1.0, 
                                                    orbclient::Color::rgb(255,0,255),
                                                    &mut window);
        
        window.sync();
    }
}