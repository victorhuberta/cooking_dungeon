use std::cmp;

use crate::prelude::*;

pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = Self {
            map: Map::new(WORLD_WIDTH, WORLD_HEIGHT),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_rooms(NUM_ROOMS, rng);
        mb.build_corridors(rng);
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

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let curr = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, curr.x, prev.y);
                self.apply_vertical_tunnel(prev.y, curr.y, curr.x);
            } else {
                self.apply_vertical_tunnel(prev.y, curr.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, curr.x, curr.y);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
            if let Some(idx) = self.map.idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
            if let Some(idx) = self.map.idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
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

    pub fn render_info(&self, camera: &Camera) -> Vec<RenderInfo> {
        let mut info = Vec::new();
        for y in camera.top()..=camera.bottom() {
            for x in camera.left()..=camera.right() {
                if let Some(tile) = self.tile(Point::new(x, y)) {
                    match tile {
                        TileType::Floor => info.push(RenderInfo {
                            x: x - camera.left(),
                            y: y - camera.top(),
                            fg: WHITE,
                            bg: BLACK,
                            glyph: to_cp437('.'),
                        }),
                        TileType::Wall => info.push(RenderInfo {
                            x: x - camera.left(),
                            y: y - camera.top(),
                            fg: WHITE,
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
        self.tile(position)
            .map_or(false, |tile| *tile == TileType::Floor)
    }

    fn tile(&self, position: Point) -> Option<&TileType> {
        self.idx(position).and_then(|idx| self.tiles.get(idx))
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
