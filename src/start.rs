#![allow(unused_variables)]

extern crate sdl2;

/*struct SDL_handles<'a> {
    context: sdl2::sdl::Sdl,
    video: sdl2::sdl::VideoSubsystem,
    renderer: sdl2::render::Renderer<'a>,
}*/

pub fn bootstrap<'a>(win_width: i32, win_height: i32, win_name: &str) -> (sdl2::render::Renderer<'a>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap(); // context
    let sdl_video = sdl_context.video().unwrap(); // video
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Make a new window
    let window = sdl_video.window(win_name, 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed on creating a new window!");
    
    // turn window into a renderer, cannot do anything with window from now on.
    let mut renderer = window.renderer().build().unwrap(); 

    (renderer, event_pump)
}