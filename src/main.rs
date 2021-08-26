#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

mod controller;
mod domain;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Cooking Dungeon")
        .build()?;
    main_loop(ctx, controller::State::new())
}
