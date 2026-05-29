use crate::game::state::player::Player;
use crate::game::state::map::tile_at;
use micromath::F32Ext;

#[derive(Clone, Copy)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub range: f32,
    pub speed: f32,
    pub hp: i32,
    pub attack_ticks: u32,
}

impl Enemy {
    pub const DEFAULT: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            range: 8.0,
            speed: 0.08,
            hp: 2,
            attack_ticks: 0,
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        let dx = player.x - self.x;
        let dy = player.y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < self.range && dist > 0.4 {
            let move_x = (dx / dist) * self.speed;
            let move_y = (dy / dist) * self.speed;
            self.try_move(move_x, move_y);
        }

        if self.attack_ticks > 0 {
            self.attack_ticks -= 1;
        }
        
        if dist < 0.9 && self.attack_ticks == 0 {
            player.hp -= 1;
            self.attack_ticks = 30;
        }
    }

    fn try_move(&mut self, dx: f32, dy: f32) {
        let nx = self.x + dx;
        let ny = self.y + dy;
        if !is_wall(nx, self.y) {
            self.x = nx;
        }
        if !is_wall(self.x, ny) {
            self.y = ny;
        }
    }
}

fn is_wall(x: f32, y: f32) -> bool {
    tile_at(x as i32, y as i32) != 0
}
