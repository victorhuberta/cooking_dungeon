use bracket_lib::prelude::*;

use crate::domain::*;

pub struct Map {
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new(tiles: Vec<TileType>) -> Self {
        Self { tiles }
    }

    pub fn render_info(&self) -> MapRenderInfo {
        let mut info = vec![];
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => info.push((x, y, YELLOW, BLACK, to_cp437('.'))),
                    TileType::Wall => info.push((x, y, GREEN, BLACK, to_cp437('#'))),
                }
            }
        }
        info
    }
}

fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

#[derive(Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

pub type MapRenderInfo = Vec<RenderInfo>;
