use crate::prelude::*;

pub struct Player {
    position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn update(&mut self, key: Option<VirtualKeyCode>, map: &Map, camera: &mut Camera) {
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
                camera.on_player_move(new_position);
            }
        }
    }

    pub fn render_info(&self, camera: &mut Camera) -> RenderInfo {
        RenderInfo {
            x: self.position.x - camera.left(),
            y: self.position.y - camera.top(),
            fg: WHITE,
            bg: BLACK,
            glyph: to_cp437('@'),
        }
    }
}
