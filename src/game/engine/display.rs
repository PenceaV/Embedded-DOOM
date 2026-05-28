use embedded_graphics::{
    prelude::*, pixelcolor::Rgb565, draw_target::DrawTarget, primitives::Rectangle,
};
use crate::game::state::player::Player;
use crate::game::state::enemy::Enemy;
use crate::game::engine::{
    raycaster::cast_ray,
    renderer::{wall_slice, wall_color, ceiling_color, floor_color},
    sprites::{blit_sprite_column, SPRITE_SCALE, SPRITE_HAND_GUN_BACKGROUND, SPRITE_GUN_SHOOT, SPRITE_MOVE_DOWN, SPRITE_ENEMY},
};

const SCREEN_WIDTH:  usize = 320;
const SCREEN_HEIGHT: usize = 240;
const VIRTUAL_COLS:  usize = 80;
const SCALE:         usize = SCREEN_WIDTH / VIRTUAL_COLS;

const SPRITE_SIZE: i32 = 16 * SPRITE_SCALE;
const SPRITE_X:    i32 = (SCREEN_WIDTH as i32 - SPRITE_SIZE) / 2;
const SPRITE_Y:    i32 = SCREEN_HEIGHT as i32 - SPRITE_SIZE;

pub struct GameDisplay<T> {
    screen: T,
    col_buf: [Rgb565; SCREEN_HEIGHT],
    depth_buffer: [f32; VIRTUAL_COLS],
}

impl<T, E> GameDisplay<T>
where
    T: DrawTarget<Color = Rgb565, Error = E>,
{
    pub fn new(screen: T) -> Self {
        Self {
            screen,
            col_buf: [Rgb565::new(0, 0, 0); SCREEN_HEIGHT],
            depth_buffer: [0.0; VIRTUAL_COLS],
        }
    }

    pub fn render(&mut self, player: &Player, enemies: &[Enemy]) -> Result<(), E> {
        for col in 0..VIRTUAL_COLS {
            let camera_x = 2.0 * col as f32 / VIRTUAL_COLS as f32 - 1.0;
            let hit = cast_ray(player, camera_x);
            let slice = wall_slice(hit.perp_dist);
            self.depth_buffer[col] = hit.perp_dist;

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

            for enemy in enemies {
                let rx = enemy.x - player.x;
                let ry = enemy.y - player.y;

                let inv_det = 1.0 / (player.plane_x * player.dir_y - player.dir_x * player.plane_y);
                let transform_x = inv_det * (player.dir_y * rx - player.dir_x * ry);
                let transform_y = inv_det * (-player.plane_y * rx + player.plane_x * ry);

                if transform_y > 0.1 {
                    let sprite_screen_x = ((VIRTUAL_COLS as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;
                    let full_height = (SCREEN_HEIGHT as f32 / transform_y).abs() as i32;
                    let sprite_height = (full_height as f32 * 0.6) as i32;
                    let sprite_width = sprite_height / SCALE as i32; 
                    
                    let draw_start_x = sprite_screen_x - sprite_width / 2;
                    let draw_end_x = sprite_screen_x + sprite_width / 2;

                    if (col as i32) >= draw_start_x && (col as i32) < draw_end_x {
                        if transform_y < self.depth_buffer[col] {
                            let sprite_col = (col as i32 - draw_start_x) * 16 / sprite_width;
                            let draw_start_y = (SCREEN_HEIGHT as i32 / 2) + (full_height / 2) - sprite_height;
                            let mut drawn = false;

                            for row in 0..16 {
                                let pixel = SPRITE_ENEMY[(row * 16 + sprite_col as i32) as usize];
                                if let crate::game::engine::sprites::SpritePixel::Color(color) = pixel {
                                    drawn = true;
                                    let y_start = draw_start_y + (row * sprite_height / 16);
                                    let y_end = draw_start_y + ((row + 1) * sprite_height / 16);
                                    
                                    for screen_y in y_start..y_end {
                                        if screen_y >= 0 && screen_y < SCREEN_HEIGHT as i32 {
                                            self.col_buf[screen_y as usize] = color;
                                        }
                                    }
                                }
                            }
                            if drawn {
                                self.depth_buffer[col] = transform_y;
                            }
                        }
                    }
                }
            }

            let screen_x = (col * SCALE) as i32;
            let in_sprite_col = screen_x >= SPRITE_X && screen_x < SPRITE_X + SPRITE_SIZE;
            if in_sprite_col {
                let sprite = if player.is_shooting() {
                    &SPRITE_GUN_SHOOT
                } else if player.walking_ticks > 0 {
                    if (player.walking_ticks / 2) % 2 == 1 {
                        &SPRITE_MOVE_DOWN
                    } else {
                        &SPRITE_HAND_GUN_BACKGROUND
                    }
                } else {
                    &SPRITE_HAND_GUN_BACKGROUND
                };

                blit_sprite_column(
                    sprite,
                    &mut self.col_buf,
                    SPRITE_X,
                    SPRITE_Y,
                    screen_x,
                );
            }

            let screen_y = (SCREEN_WIDTH - 1) as i32 - screen_x;

            for offset in 0..SCALE {
                let bbox = Rectangle::new(
                    Point::new(0, screen_y - offset as i32),
                    Size::new(SCREEN_HEIGHT as u32, 1),
                );
                self.screen.fill_contiguous(&bbox, self.col_buf.iter().copied())?;
            }
        }
        Ok(())
    }
}
