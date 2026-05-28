use embedded_graphics::pixelcolor::Rgb565;
use crate::game::engine::raycaster::RayHit;

const BLACK: Rgb565 = Rgb565::new(0,  0,  0);
const WALL_EDGE: Rgb565 = Rgb565::new(0, 63,  0);
const WALL_MAIN: Rgb565 = Rgb565::new(0, 40,  0);
const WALL_ALT: Rgb565  = Rgb565::new(0, 50,  0);
const FLOOR_DOT: Rgb565 = Rgb565::new(0, 16,  0);

const SCREEN_HEIGHT: i32 = 240;
const HALF_HEIGHT: i32 = SCREEN_HEIGHT / 2;

pub struct WallSlice {
    pub start: i32,
    pub end: i32,
}

pub fn wall_slice(perp_dist: f32) -> WallSlice {
    let line_height = if perp_dist > 0.01 {
        (SCREEN_HEIGHT as f32 / perp_dist) as i32
    } else {
        SCREEN_HEIGHT
    };
    WallSlice {
        start: (HALF_HEIGHT - line_height / 2).max(0),
        end: (HALF_HEIGHT + line_height / 2).min(SCREEN_HEIGHT - 1),
    }
}

pub fn wall_color(hit: &RayHit, y: i32, slice: &WallSlice, col: usize) -> Rgb565 {
    if (y == slice.start) || (y == slice.end) || is_texture_edge(hit.wall_x) {
        return WALL_EDGE;
    }
    match hit.tile {
        2 => stripe_color(col),
        3 => checker_color(col, y),
        4 => band_color(y, slice),
        _ => WALL_MAIN,
    }
}

pub fn ceiling_color() -> Rgb565 {
    BLACK
}

pub fn floor_color(col: usize, row: i32) -> Rgb565 {
    if col % 4 == 0 && row % 12 == 0 { FLOOR_DOT } else { BLACK }
}

fn is_texture_edge(wall_x: f32) -> bool {
    let shifted = wall_x + 0.5;
    let centered = wall_x - (shifted as i32) as f32 + 0.5;
    (centered - 0.5).abs() <= 0.05
}

fn stripe_color(col: usize) -> Rgb565 {
    if col % 2 == 0 { WALL_ALT } else { BLACK }
}

fn checker_color(col: usize, row: i32) -> Rgb565 {
    if (col ^ row as usize) & 1 == 0 { WALL_ALT } else { BLACK }
}

fn band_color(y: i32, slice: &WallSlice) -> Rgb565 {
    if y <= slice.start + 4 || y >= slice.end - 4 { WALL_ALT } else { BLACK }
}