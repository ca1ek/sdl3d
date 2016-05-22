use orbclient;
use super::geometry;
use super::texture;
pub mod wireframe;

pub trait RenderFn {
    fn triangle_s<T: texture::GetColor>(triangle: &geometry::Triangle, 
                  color: orbclient::Color, 
                  window: &mut orbclient::window::Window,
                  texture: &T);
}