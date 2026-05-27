// Name: display.rs
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    draw_target::DrawTarget,
};
use crate::game::player::Player;

const BUF_WIDTH: usize = 80;
const BUF_HEIGHT: usize = 60;
const SCALE_FACTOR: usize = 4; 

pub struct GameDisplay<T> {
    screen: T,
    column_buffer: [Rgb565; 240],
}

impl<T, E> GameDisplay<T>
where
    T: DrawTarget<Color = Rgb565, Error = E>,
{
    pub fn new(screen: T) -> Self {
        Self {
            screen,
            column_buffer: [Rgb565::BLACK; 240],
        }
    }

    pub fn render_scene(&mut self, player: &Player, world_map: &[[i32; 24]; 24]) -> Result<(), E> {
        let f_width = BUF_WIDTH as f32;
        let edge_tolerance: f32 = 0.05;

        for x_virt in 0..BUF_WIDTH {
            let camera_x = 2.0 * (x_virt as f32) / f_width - 1.0;
            let ray_dir_x = player.dir_x + player.plane_x * camera_x;
            let ray_dir_y = player.dir_y + player.plane_y * camera_x;

            let mut map_x = player.x as i32;
            let mut map_y = player.y as i32;

            let mut side_dist_x;
            let mut side_dist_y;

            let abs_dir_x = if ray_dir_x < 0.0 { -ray_dir_x } else { ray_dir_x };
            let abs_dir_y = if ray_dir_y < 0.0 { -ray_dir_y } else { ray_dir_y };

            let delta_dist_x = if ray_dir_x == 0.0 { f32::INFINITY } else { 1.0 / abs_dir_x };
            let delta_dist_y = if ray_dir_y == 0.0 { f32::INFINITY } else { 1.0 / abs_dir_y };
            let perp_wall_dist;

            let step_x;
            let step_y;
            let mut hit = 0;
            let mut side = 0;

            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (player.x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f32 + 1.0 - player.x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (player.y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f32 + 1.0 - player.y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }
                if map_x >= 0 && map_x < 24 && map_y >= 0 && map_y < 24 {
                    if world_map[map_x as usize][map_y as usize] > 0 { hit = 1; }
                } else {
                    break;
                }
            }

            if side == 0 { perp_wall_dist = side_dist_x - delta_dist_x; }
            else { perp_wall_dist = side_dist_y - delta_dist_y; }

            let line_height = if perp_wall_dist > 0.01 { (240.0 / perp_wall_dist) as i32 } else { 240 };

            let mut start = -line_height / 2 + 120;
            if start < 0 { start = 0; }
            let mut end = line_height / 2 + 120;
            if end >= 240 { end = 239; }

            let mut wall_x = if side == 0 {
                player.y + perp_wall_dist * ray_dir_y
            } else {
                player.x + perp_wall_dist * ray_dir_x
            };
            wall_x -= wall_x as i32 as f32;

            let tex = if map_x >= 0 && map_x < 24 && map_y >= 0 && map_y < 24 {
                world_map[map_x as usize][map_y as usize]
            } else { 1 };

            let rounded_wall_x = ((wall_x + 0.5) as i32) as f32;
            let diff = wall_x - rounded_wall_x;
            let abs_diff = if diff < 0.0 { -diff } else { diff };
            let is_edge = abs_diff <= edge_tolerance;

            for y_native in 0..240 {
                if y_native < start {
                    self.column_buffer[y_native as usize] = Rgb565::BLACK;
                } else if y_native >= start && y_native <= end {
                    if is_edge || y_native == start || y_native == end {
                        self.column_buffer[y_native as usize] = Rgb565::GREEN;
                    } else {
                        let y_virt = y_native / SCALE_FACTOR as i32;
                        match tex {
                            2 => {
                                if x_virt % 2 == 0 { self.column_buffer[y_native as usize] = Rgb565::GREEN; }
                                else { self.column_buffer[y_native as usize] = Rgb565::BLACK; }
                            }
                            3 => {
                                if (x_virt ^ y_virt as usize) & 1 == 0 { self.column_buffer[y_native as usize] = Rgb565::GREEN; }
                                else { self.column_buffer[y_native as usize] = Rgb565::BLACK; }
                            }
                            4 => {
                                if y_native <= start + 4 || y_native >= end - 4 {
                                    self.column_buffer[y_native as usize] = Rgb565::GREEN;
                                } else {
                                    self.column_buffer[y_native as usize] = Rgb565::BLACK;
                                }
                            }
                            _ => self.column_buffer[y_native as usize] = Rgb565::GREEN,
                        }
                    }
                } else {
                    if (x_virt % 4 == 0) && (y_native % 12 == 0) {
                        self.column_buffer[y_native as usize] = Rgb565::new(0, 16, 0);
                    } else {
                        self.column_buffer[y_native as usize] = Rgb565::BLACK;
                    }
                }
            }

            let current_x = x_virt * SCALE_FACTOR;
            for x_offset in 0..SCALE_FACTOR {

                let virtual_x = current_x + x_offset;
                let bounding_box = embedded_graphics::primitives::Rectangle::new(
                    Point::new(0, virtual_x as i32),
                    Size::new(240, 1)
                );
                
                self.screen.fill_contiguous(&bounding_box, self.column_buffer.iter().copied())?;
            }
        }
        Ok(())
    }
}