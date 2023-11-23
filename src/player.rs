use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,
    y: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "A";
    }
}