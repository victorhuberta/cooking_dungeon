use crate::prelude::*;

pub struct State {
    map: Map,
    player: Player,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx.key, &self.map);
        for tile in self.map.render_info() {
            self.render(ctx, tile);
        }
        self.render(ctx, self.player.render_info());
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(vec![TileType::Floor; MAP_TILES_N]),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, info: RenderInfo) {
        let (x, y, fg, bg, symbol) = info;
        ctx.set(x, y, fg, bg, symbol);
    }
}
