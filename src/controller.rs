use crate::prelude::*;

pub struct State {
    world: World,
    resources: Resources,
    systems: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);

        let mut world = World::default();
        spawn_player(&mut world, mb.player_start());
        spawn_random_enemies(&mut world, &mb, &mut rng);

        let mut resources = Resources::default();
        resources.insert(mb.map_clone());
        resources.insert(Camera::new(mb.player_start()));

        Self {
            world,
            resources,
            systems: build_scheduler(),
        }
    }
}
