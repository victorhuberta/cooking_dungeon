use crate::prelude::*;

pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = Self {
            map: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_rooms(NUM_ROOMS, rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    pub fn map_clone(&self) -> Map {
        self.map.clone()
    }

    pub fn player_start(&self) -> Point {
        self.player_start
    }

    fn fill(&mut self, tile_type: TileType) {
        for idx in 0..self.map.tiles.len() {
            self.map.tiles[idx] = tile_type;
        }
    }

    fn build_rooms(&mut self, num_rooms: usize, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < num_rooms {
            let room = Rect::with_size(
                rng.range(1, self.map.width - 10),
                rng.range(1, self.map.height - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for existing_room in &self.rooms {
                if existing_room.intersect(&room) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                room.for_each(|position| {
                    if let Some(idx) = self.map.idx(position) {
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }
}

#[derive(Clone)]
pub struct Map {
    width: i32,
    height: i32,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Floor; (width * height) as usize],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn render_info(&self) -> Vec<RenderInfo> {
        let mut info = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.tile(Point::new(x, y)) {
                    match tile {
                        TileType::Floor => info.push(RenderInfo {
                            x,
                            y,
                            fg: BLACK,
                            bg: BROWN2,
                            glyph: to_cp437('.'),
                        }),
                        TileType::Wall => info.push(RenderInfo {
                            x,
                            y,
                            fg: BLACK,
                            bg: BLACK,
                            glyph: to_cp437('#'),
                        }),
                    }
                }
            }
        }
        info
    }

    pub fn can_enter_tile(&self, position: Point) -> bool {
        if let Some(tile) = self.tile(position) {
            *tile == TileType::Floor
        } else {
            false
        }
    }

    fn tile(&self, position: Point) -> Option<&TileType> {
        if let Some(idx) = self.idx(position) {
            self.tiles.get(idx)
        } else {
            None
        }
    }

    // Return an index only if the position is in bounds.
    fn idx(&self, position: Point) -> Option<usize> {
        if position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
        {
            Some(((position.y * self.width) + position.x) as usize)
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
