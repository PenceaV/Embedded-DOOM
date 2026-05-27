pub struct Player {
    pub x: i32,
    pub y: i32,
    pub last_x: i32,
    pub last_y: i32,
    pub size: u32,
    max_width: i32,
    max_height: i32,
}

impl Player {
    pub fn new(size: u32, screen_width: i32, screen_height: i32) -> Self {
        let initial_x = (screen_width - size as i32) / 2;
        let initial_y = (screen_height - size as i32) / 2;
        Self {
            x: initial_x,
            y: initial_y,
            last_x: initial_x,
            last_y: initial_y,
            size,
            max_width: screen_width,
            max_height: screen_height,
        }
    }

    pub fn move_left(&mut self, step: i32) {
        self.x = (self.x - step).max(0);
    }

    pub fn move_right(&mut self, step: i32) {
        self.x = (self.x + step).min(self.max_width - self.size as i32);
    }

    pub fn move_up(&mut self, step: i32) {
        self.y = (self.y - step).max(0);
    }

    pub fn move_down(&mut self, step: i32) {
        self.y = (self.y + step).min(self.max_height - self.size as i32);
    }

    pub fn has_moved(&self) -> bool {
        self.x != self.last_x || self.y != self.last_y
    }

    pub fn update_last_position(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
    }
}