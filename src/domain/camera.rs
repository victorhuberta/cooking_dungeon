use crate::prelude::*;

pub struct Camera {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let mut camera = Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        camera.on_player_move(player_position);
        camera
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn right(&self) -> i32 {
        self.right
    }

    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left = player_position.x - DISPLAY_WIDTH / 2;
        self.top = player_position.y - DISPLAY_HEIGHT / 2;
        self.right = player_position.x + DISPLAY_WIDTH / 2;
        self.bottom = player_position.y + DISPLAY_HEIGHT / 2;
    }
}
