use crate::prelude::*;

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
                if let Some(tile) = self.tile(Point::new(x, y)) {
                    match tile {
                        TileType::Floor => info.push((x, y, YELLOW, BLACK, to_cp437('.'))),
                        TileType::Wall => info.push((x, y, GREEN, BLACK, to_cp437('#'))),
                    }
                }
            }
        }
        info
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        if let Some(tile) = self.tile(point) {
            tile == TileType::Floor
        } else {
            false
        }
    }

    fn tile(&self, point: Point) -> Option<TileType> {
        let idx = ((point.y * SCREEN_WIDTH) + point.x) as usize;
        if let Some(tile) = self.tiles.get(idx) {
            Some(*tile)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

pub type MapRenderInfo = Vec<RenderInfo>;
