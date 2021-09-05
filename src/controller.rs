use crate::prelude::*;

pub struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(ctx.key, &self.map, &mut self.camera);

        ctx.set_active_console(0);
        for tile in self.map.render_info(&self.camera) {
            render(ctx, tile);
        }
        ctx.set_active_console(1);
        render(ctx, self.player.render_info(&mut self.camera));
    }
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map_clone(),
            player: Player::new(mb.player_start()),
            camera: Camera::new(mb.player_start()),
        }
    }
}

fn render(ctx: &mut BTerm, ri: RenderInfo) {
    ctx.set(ri.x, ri.y, ri.fg, ri.bg, ri.glyph);
}
