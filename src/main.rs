mod framebuffer;
mod game_of_life;
mod render;

use framebuffer::Framebuffer;
use game_of_life::{BEACON, BLINKER, GLIDER, GameOfLife, TOAD, PULSAR, GOSPER_GLIDER_GUN, ACORN};
use raylib::prelude::*;
use render::draw_game_of_life;
use std::thread;
use std::time::Duration;

fn main() {
    const WINDOW_WIDTH: i32 = 500;
    const WINDOW_HEIGHT: i32 = 500;
    const CELL_SIZE: u32 = 5;
    const FRAMES_PER_GENERATION: u32 = 5;

    let (mut window, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, Color::BLACK);
    let mut game = GameOfLife::new(
        WINDOW_WIDTH as usize / CELL_SIZE as usize,
        WINDOW_HEIGHT as usize / CELL_SIZE as usize,
    );


    game.place_pattern(BLINKER, 25, 15);
    game.place_pattern(TOAD, 40, 25);
    game.place_pattern(BEACON, 60, 35);
    game.place_pattern(GOSPER_GLIDER_GUN, 0, 0);
    game.place_pattern(ACORN, 50, 51);
    game.place_pattern(PULSAR, 10, 10);    
    game.place_pattern(PULSAR, 80, 80); 
    game.place_pattern(PULSAR, 10, 80); 
    game.place_pattern(PULSAR, 80, 10);
    game.place_pattern(PULSAR, 10, 50); 
    game.place_pattern(PULSAR, 80, 50);   
    game.place_pattern(GLIDER, 20, 20);
    game. place_pattern(GLIDER, 30, 30);
    game.place_pattern(GLIDER, 40, 40);
    game.place_pattern(GLIDER, 50, 50);
    game.place_pattern(GLIDER, 60, 60);
    game.place_pattern(GLIDER, 70, 70);
    game.place_pattern(GOSPER_GLIDER_GUN, 80, 80);

    let mut frame_counter = 0;

    while !window.window_should_close() {
        framebuffer.clear();

        draw_game_of_life(
            &mut framebuffer,
            &game,
            Vector2::zero(),
            CELL_SIZE,
            Color::GREEN,
        );

        frame_counter += 1;
        if frame_counter >= FRAMES_PER_GENERATION {
            game.step();
            frame_counter = 0;
        }

        framebuffer.swap_buffers(&mut window, &thread);
        thread::sleep(Duration::from_millis(16));
    }
}
