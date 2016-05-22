use orbclient;
use super::geometry;
use super::texture;

pub mod wireframe;
pub mod textured;
pub mod filled;

pub trait RenderFn {
    fn triangle<T: texture::GetColor>(triangle: &geometry::Triangle, 
                  color: orbclient::Color, 
                  window: &mut orbclient::window::Window,
                  texture: &T);
}