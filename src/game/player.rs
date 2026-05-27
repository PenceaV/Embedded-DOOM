// Name: player.rs
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
            x: 22.0,
            y: 12.0,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }

    pub fn move_forward(&mut self, move_speed: f32, world_map: &[[i32; 24]; 24]) {
        let new_x = self.x + self.dir_x * move_speed;
        if world_map[new_x as usize][self.y as usize] == 0 { self.x = new_x; }
        let new_y = self.y + self.dir_y * move_speed;
        if world_map[self.x as usize][new_y as usize] == 0 { self.y = new_y; }
    }

    pub fn move_backward(&mut self, move_speed: f32, world_map: &[[i32; 24]; 24]) {
        let new_x = self.x - self.dir_x * move_speed;
        if world_map[new_x as usize][self.y as usize] == 0 { self.x = new_x; }
        let new_y = self.y - self.dir_y * move_speed;
        if world_map[self.x as usize][new_y as usize] == 0 { self.y = new_y; }
    }

    pub fn rotate(&mut self, rot_speed: f32) {
        // Fast Taylor Series Approximation for Sin/Cos without requiring libm
        let cos_a = 1.0 - (rot_speed * rot_speed) / 2.0;
        let sin_a = rot_speed - (rot_speed * rot_speed * rot_speed) / 6.0;

        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * cos_a - self.dir_y * sin_a;
        self.dir_y = old_dir_x * sin_a + self.dir_y * cos_a;

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * cos_a - self.plane_y * sin_a;
        self.plane_y = old_plane_x * sin_a + self.plane_y * cos_a;
    }
}