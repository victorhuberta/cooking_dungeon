#![warn(clippy::all, clippy::pedantic)]

mod controller;
mod domain;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::domain::*;
}

use crate::prelude::*;

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Cooking Dungeon")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        // NOTE: The resources directory must be named "resources"
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(WORLD_WIDTH, WORLD_HEIGHT, "terminal8x8.png")
        .build()?;
    main_loop(ctx, controller::State::new())
}
