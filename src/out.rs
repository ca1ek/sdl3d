extern crate sdl2;

pub struct Renderer<'a> {
    renderer: sdl2::render::Renderer<'a> // replace that if porting with any struct that can draw onto the screen
}

impl Renderer<'a> {

}
