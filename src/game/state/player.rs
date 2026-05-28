use crate::game::state::map::tile_at;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir_x: f32,
    pub dir_y: f32,
    pub plane_x: f32,
    pub plane_y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 2.0,
            y: 2.0,
            dir_x: 1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }

    pub fn move_forward(&mut self, speed: f32) {
        self.try_move(self.dir_x * speed, self.dir_y * speed);
    }

    pub fn move_backward(&mut self, speed: f32) {
        self.try_move(-self.dir_x * speed, -self.dir_y * speed);
    }

    pub fn rotate(&mut self, angle: f32) {
        let cos_a = 1.0 - (angle * angle) / 2.0;
        let sin_a = angle - (angle * angle * angle) / 6.0;
        (self.dir_x, self.dir_y) = rotate_vec(self.dir_x, self.dir_y, cos_a, sin_a);
        (self.plane_x, self.plane_y) = rotate_vec(self.plane_x, self.plane_y, cos_a, sin_a);
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

fn rotate_vec(x: f32, y: f32, cos_a: f32, sin_a: f32) -> (f32, f32) {
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

fn is_wall(x: f32, y: f32) -> bool {
    tile_at(x as i32, y as i32) != 0
}