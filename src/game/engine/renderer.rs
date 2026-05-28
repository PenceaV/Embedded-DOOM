use embedded_graphics::pixelcolor::Rgb565;
use crate::game::engine::raycaster::RayHit;

const BLACK:     Rgb565 = Rgb565::new(0,  0,  0);
const CEILING:   Rgb565 = Rgb565::new(2,  4,  8);
const FLOOR:     Rgb565 = Rgb565::new(6,  4,  2);
const FLOOR_DOT: Rgb565 = Rgb565::new(10, 8,  4);
const WALL_EDGE: Rgb565 = Rgb565::new(25, 50, 25);

const STONE_MAIN: Rgb565 = Rgb565::new(15, 30, 15);
const STONE_DARK: Rgb565 = Rgb565::new(10, 20, 10);

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

pub fn wall_color(hit: &RayHit, y: i32, slice: &WallSlice, _col: usize) -> Rgb565 {
    let is_dark = hit.side == 1;
    let main = if is_dark { STONE_DARK } else { STONE_MAIN };
    let shade = if is_dark { BLACK } else { STONE_DARK };

    let wall_height = (slice.end - slice.start).max(1);
    let v = (y - slice.start) as f32 / wall_height as f32;
    let u = hit.wall_x;

    let bricks_y = 4.0;
    let bricks_x = 2.0;

    let u_off = if (v * bricks_y) as i32 % 2 == 1 { 0.5 } else { 0.0 };
    let uu = (u * bricks_x + u_off) % 1.0;
    let vv = (v * bricks_y) % 1.0;

    if uu < 0.1 || vv < 0.1 {
        shade
    } else if uu > 0.9 || vv > 0.9 {
        WALL_EDGE
    } else {
        main
    }
}

pub fn ceiling_color() -> Rgb565 {
    CEILING
}

pub fn floor_color(col: usize, row: i32) -> Rgb565 {
    if (col ^ row as usize) % 17 == 0 {
        FLOOR_DOT
    } else {
        FLOOR
    }
}
