use bracket_lib::prelude::*;

use crate::domain::*;

pub struct State {
    map: Map,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        for tile in self.map.render_info() {
            let (x, y, fg, bg, symbol) = tile;
            ctx.set(x, y, fg, bg, symbol);
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(vec![TileType::Floor; MAP_TILES_N]),
        }
    }
}
