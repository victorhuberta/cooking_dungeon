use crate::prelude::*;

pub struct State {
    world: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    enemy_systems: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);

        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.world, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.world, &mut self.resources),
            TurnState::EnemyTurn => self
                .enemy_systems
                .execute(&mut self.world, &mut self.resources),
        }

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
        resources.insert(TurnState::AwaitingInput);

        Self {
            world,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            enemy_systems: build_enemy_scheduler(),
        }
    }
}
