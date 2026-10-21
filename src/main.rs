mod framebuffer;
mod render;
mod triangle;
mod line;
mod load_objects;

use framebuffer::Framebuffer;
use render::render;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    const WINDOW_WIDTH: i32 = 500;
    const WINDOW_HEIGHT: i32 = 500;

    let (mut window, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    // Load the 3D object from an .obj file
    let obj = load_objects::load_obj("assets/object.obj").expect("Failed to load .obj file");
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, Color::BLACK);


    while !window.window_should_close() {
        framebuffer.clear();

        // Render the 3D object to the framebuffer
        render(&mut framebuffer, &obj, Vector3::new(0.0, 0.0, -5.0), 90.0);

        framebuffer.swap_buffers(&mut window, &thread);
        thread::sleep(Duration::from_millis(16));
    }
}
