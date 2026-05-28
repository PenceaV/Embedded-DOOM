use embedded_graphics::{prelude::*, pixelcolor::Rgb565, draw_target::DrawTarget, primitives::Rectangle};
use crate::game::state::player::Player;
use crate::game::engine::{
    raycaster::cast_ray,
    renderer::{wall_slice, wall_color, ceiling_color, floor_color},
};

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;
const VIRTUAL_COLS: usize = 80;
const SCALE: usize = SCREEN_WIDTH / VIRTUAL_COLS;

pub struct GameDisplay<T> {
    screen: T,
    col_buf: [Rgb565; SCREEN_HEIGHT],
}

impl<T, E> GameDisplay<T>
where
    T: DrawTarget<Color = Rgb565, Error = E>,
{
    pub fn new(screen: T) -> Self {
        Self {
            screen,
            col_buf: [Rgb565::new(0, 0, 0); SCREEN_HEIGHT],
        }
    }

    pub fn render(&mut self, player: &Player) -> Result<(), E> {
        for col in 0..VIRTUAL_COLS {
            let camera_x = 2.0 * col as f32 / VIRTUAL_COLS as f32 - 1.0;
            let hit = cast_ray(player, camera_x);
            let slice = wall_slice(hit.perp_dist);

            for row in 0..SCREEN_HEIGHT {
                let y = row as i32;
                self.col_buf[row] = if y < slice.start {
                    ceiling_color()
                } else if y <= slice.end {
                    wall_color(&hit, y, &slice, col)
                } else {
                    floor_color(col, y)
                };
            }

            let screen_x = (col * SCALE) as i32;
            for offset in 0..SCALE {
                let bbox = Rectangle::new(
                    Point::new(0, screen_x + offset as i32),
                    Size::new(SCREEN_HEIGHT as u32, 1),
                );
                self.screen.fill_contiguous(&bbox, self.col_buf.iter().copied())?;
            }
        }
        Ok(())
    }
}