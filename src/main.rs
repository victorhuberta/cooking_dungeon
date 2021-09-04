#![warn(clippy::all, clippy::pedantic)]

mod controller;
mod domain;
mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::domain::*;
}

use crate::prelude::*;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Cooking Dungeon")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(ctx, controller::State::new())
}
