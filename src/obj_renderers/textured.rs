use super::super::renderers::textured;
use super::super::texture;
use super::super::geometry;
use super::RenderFn;
use orbclient;

/// An object for the textured renderer
pub struct Renderer {
    //empty
}

impl RenderFn for Renderer {
    fn triangle<T: texture::GetColor>(triangle: &geometry::Triangle, 
                  color: orbclient::Color, 
                  window: &mut orbclient::window::Window,
                  texture: &T) {
    
        textured::triangle_s(triangle, color, window, texture);
    }
}