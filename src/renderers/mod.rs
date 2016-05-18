pub mod wireframe;
pub mod filled;

/// Do perspective calculation on a point, return (x, y).
fn perpective(x: f32, y: f32, z: f32) -> (f32, f32) {
    (x/z, y/z)
}

/// Turn floating point coordinates to something drawable on the screen, return (x, y)
fn screen(x: f32, y: f32, screen_width: i32, screen_height: i32) -> (i32, i32) {
    ((x * (screen_height/2) as f32) as i32 + (screen_width/2),
     (y * (screen_height/2) as f32) as i32 + (screen_height/2))
}