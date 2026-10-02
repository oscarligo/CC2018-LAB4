use raylib::prelude::*;
use crate::line::line;
use crate::framebuffer::Framebuffer;

pub struct Polygon {
    pub vertices: Vec<Vector2>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Polygon { vertices }
    }
    
    pub fn draw_outline(&self, framebuffer: &mut Framebuffer, color: Color) {
        framebuffer.set_current_color(color);
        let num_vertices = self.vertices.len();
        for i in 0..num_vertices {
            let start = self.vertices[i];
            let end = self.vertices[(i + 1) % num_vertices]; // Conecta el último con el primero
            line(framebuffer, start, end);
        }
    }

}