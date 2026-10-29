mod fragment;
mod framebuffer;
mod line;
mod obj;
mod triangle;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use obj::Obj;
use triangle::{rasterize, Triangle, TriangleMode};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const MARGIN: f32 = 0.8;

fn model_triangles(model: &Obj, color: Color) -> Vec<Triangle> {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for v in &model.vertices {
        min_x = min_x.min(v.x);
        max_x = max_x.max(v.x);
        min_y = min_y.min(v.y);
        max_y = max_y.max(v.y);
    }

    let scale = (WIDTH as f32 * MARGIN / (max_x - min_x))
        .min(HEIGHT as f32 * MARGIN / (max_y - min_y));
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let to_screen = |v: Vector3| {
        Vector2::new(
            (v.x - center_x) * scale + WIDTH as f32 / 2.0,
            HEIGHT as f32 / 2.0 - (v.y - center_y) * scale,
        )
    };

    let mut triangles = Vec::new();

    for [i0, i1, i2] in &model.indices {
        let a = model.vertices[*i0];
        let b = model.vertices[*i1];
        let c = model.vertices[*i2];

        triangles.push(Triangle::new(to_screen(a), to_screen(b), to_screen(c), color));
    }

    triangles
}

fn main() {
    let model = Obj::load("models/nave.obj");
    println!("Vertices: {}, triangulos: {}", model.vertices.len(), model.indices.len());

    let mut framebuffer = Framebuffer::new(WIDTH as u32, HEIGHT as u32, Color::BLACK);

    let triangles = model_triangles(&model, Color::WHITE);

    framebuffer.draw_fragments(&rasterize(&triangles, TriangleMode::Wireframe));
    framebuffer.render_to_file("output.png");
}
