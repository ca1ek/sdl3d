use super::super::renderers::filled;
use super::super::texture;
use super::super::geometry;
use super::RenderFn;
use orbclient;

/// An object for the wireframe renderer
pub struct Renderer {
    //empty
}

impl RenderFn for Renderer {
    fn triangle<T: texture::GetColor>(triangle: &geometry::Triangle, 
                  color: orbclient::Color, 
                  window: &mut orbclient::window::Window,
                  texture: &T) {
    
        wireframe::triangle_s(triangle, color, window);
    }
}