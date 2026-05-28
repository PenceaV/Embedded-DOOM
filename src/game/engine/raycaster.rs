use crate::game::state::player::Player;
use crate::game::state::map::tile_at;

pub struct RayHit {
    pub perp_dist: f32,
    pub wall_x: f32,
    pub tile: i32,
    pub side: u8,
}

pub fn cast_ray(player: &Player, camera_x: f32) -> RayHit {
    let ray_dir_x = player.dir_x + player.plane_x * camera_x;
    let ray_dir_y = player.dir_y + player.plane_y * camera_x;

    let mut map_x = player.x as i32;
    let mut map_y = player.y as i32;

    let delta_x = if ray_dir_x == 0.0 { f32::INFINITY } else { (1.0 / ray_dir_x).abs() };
    let delta_y = if ray_dir_y == 0.0 { f32::INFINITY } else { (1.0 / ray_dir_y).abs() };

    let (step_x, mut side_dist_x) = step_and_initial(ray_dir_x, player.x, map_x, delta_x);
    let (step_y, mut side_dist_y) = step_and_initial(ray_dir_y, player.y, map_y, delta_y);

    let mut side: u8;

    loop {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_y;
            map_y += step_y;
            side = 1;
        }
        if tile_at(map_x, map_y) > 0 {
            break;
        }
    }

    let perp_dist = if side == 0 {
        side_dist_x - delta_x
    } else {
        side_dist_y - delta_y
    };

    let wall_x = {
        let hit_pos = if side == 0 {
            player.y + perp_dist * ray_dir_y
        } else {
            player.x + perp_dist * ray_dir_x
        };
        hit_pos - (hit_pos as i32) as f32
    };

    RayHit {
        perp_dist,
        wall_x,
        tile: tile_at(map_x, map_y),
        side,
    }
}

fn step_and_initial(ray_dir: f32, pos: f32, map: i32, delta: f32) -> (i32, f32) {
    if ray_dir < 0.0 {
        (-1, (pos - map as f32) * delta)
    } else {
        (1, (map as f32 + 1.0 - pos) * delta)
    }
}
