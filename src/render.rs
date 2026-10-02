use crate::framebuffer::Framebuffer;
use crate::game_of_life::GameOfLife;
use raylib::prelude::*;

pub fn draw_game_of_life(
    framebuffer: &mut Framebuffer,
    game: &GameOfLife,
    position: Vector2,
    cell_size: u32,
    color: Color,
) {
    if cell_size == 0 {
        return;
    }

    framebuffer.set_current_color(color);

    for (cell_x, cell_y) in game.live_cells() {
        let base_x = position.x as i32 + cell_x as i32 * cell_size as i32;
        let base_y = position.y as i32 + cell_y as i32 * cell_size as i32;

        for offset_y in 0..cell_size as i32 {
            for offset_x in 0..cell_size as i32 {
                let pixel_x = base_x + offset_x;
                let pixel_y = base_y + offset_y;

                if pixel_x >= 0 && pixel_y >= 0 {
                    framebuffer.set_pixel(pixel_x as u32, pixel_y as u32);
                }
            }
        }
    }
}
