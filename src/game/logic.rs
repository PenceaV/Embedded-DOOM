use crate::game::state::player::Player;
use crate::game::state::enemy::Enemy;
use crate::game::input::controls::Controls;
use crate::game::engine::raycaster::cast_ray;
use crate::game::engine::audio::MusicPlayer;
use embassy_rp::pwm::Pwm;
use micromath::F32Ext;

const MOVE_SPEED: f32 = 0.35;
const ROT_SPEED: f32 = 0.15;

pub fn handle_input(controls: &Controls, player: &mut Player, enemies: &mut [Option<Enemy>; 20], enemy_count: usize) {
    if player.hp <= 0 { return; }

    if controls.forward() { player.move_forward(MOVE_SPEED); }
    if controls.backward() { player.move_backward(MOVE_SPEED); }
    if controls.turn_left() { player.rotate(ROT_SPEED); }
    if controls.turn_right() { player.rotate(-ROT_SPEED); }
    
    if controls.shoot() && !player.is_shooting() {
        player.shoot();
        handle_combat(player, enemies, enemy_count);
    }
}

fn handle_combat(player: &Player, enemies: &mut [Option<Enemy>; 20], enemy_count: usize) {
    let wall_hit = cast_ray(player, 0.0);
    let mut nearest_enemy: Option<usize> = None;
    let mut min_dist = 1000.0;

    for i in 0..enemy_count {
        if let Some(enemy) = &enemies[i] {
            let dx = enemy.x - player.x;
            let dy = enemy.y - player.y;
            let dist = (dx * dx + dy * dy).sqrt();

            let inv_det = 1.0 / (player.plane_x * player.dir_y - player.dir_x * player.plane_y);
            let transform_y = inv_det * (-player.plane_y * dx + player.plane_x * dy);
            let transform_x = inv_det * (player.dir_y * dx - player.dir_x * dy);

            if transform_y > 0.1 && transform_y < wall_hit.perp_dist {
                let angle_offset = (transform_x / transform_y).abs();
                if angle_offset < 0.2 && dist < min_dist {
                    min_dist = dist;
                    nearest_enemy = Some(i);
                }
            }
        }
    }

    if let Some(idx) = nearest_enemy {
        if let Some(ref mut enemy) = enemies[idx] {
            enemy.hp -= 1;
            if enemy.hp <= 0 {
                enemies[idx] = None;
            }
        }
    }
}

pub fn update_game_state(player: &mut Player, music: &mut MusicPlayer, pwm: &mut Pwm, enemies: &mut [Option<Enemy>; 20], enemy_count: usize) {
    player.update();
    music.update(pwm);
    for i in 0..enemy_count {
        if let Some(ref mut enemy) = enemies[i] {
            enemy.update(player);
        }
    }
}

pub fn collect_active_enemies(enemies: &[Option<Enemy>; 20], enemy_count: usize) -> &[Enemy] {
    static mut ACTIVE: [Enemy; 20] = [Enemy::DEFAULT; 20];
    let mut count = 0;
    unsafe {
        for i in 0..enemy_count {
            if let Some(enemy) = &enemies[i] {
                ACTIVE[count] = *enemy;
                count += 1;
            }
        }
        &ACTIVE[..count]
    }
}
