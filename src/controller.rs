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
            render(ctx, tile);
        }
        render(ctx, self.player.render_info());
    }
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map_clone(),
            player: Player::new(mb.player_start()),
        }
    }
}

fn render(ctx: &mut BTerm, ri: RenderInfo) {
    ctx.set(ri.x, ri.y, ri.fg, ri.bg, ri.glyph);
}
