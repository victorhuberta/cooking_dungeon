use bracket_lib::prelude::*;

use crate::domain::*;

pub struct State {
	map: Map
}

impl GameState for State {
	fn tick(&mut self, ctx: &mut BTerm) {

	}
}

impl State {
	pub fn new() -> Self {
		Self {
			map: Map::new(vec![TileType::Floor; MAP_TILES_N]),
		}
	}
}