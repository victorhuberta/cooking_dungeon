pub struct Map {
	tiles: Vec<TileType>,
}

impl Map {
	pub fn new(tiles: Vec<TileType>) -> Self {
		Self {
			tiles,
		}
	}
}

#[derive(Clone)]
pub enum TileType {
	Floor,
	Wall,
}