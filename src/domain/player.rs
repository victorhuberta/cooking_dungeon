use crate::prelude::*;

pub struct Player {
    position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn update(&mut self, key: Option<VirtualKeyCode>, map: &Map) {
        if let Some(key) = key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }

    pub fn render_info(&self) -> RenderInfo {
        (
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }
}
