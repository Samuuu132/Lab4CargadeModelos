mod fragment;
mod framebuffer;
mod line;
mod triangle;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use triangle::{rasterize, Triangle, TriangleMode};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH as u32, HEIGHT as u32, Color::BLACK);

    let triangles = vec![Triangle::new(
        Vector2::new(200.0, 450.0),
        Vector2::new(400.0, 150.0),
        Vector2::new(600.0, 450.0),
        Color::WHITE,
    )];

    framebuffer.draw_fragments(&rasterize(&triangles, TriangleMode::Wireframe));
    framebuffer.render_to_file("output.png");
}
