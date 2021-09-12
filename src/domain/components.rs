use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Player;

#[derive(Copy, Clone, PartialEq)]
pub struct Enemy;

#[derive(Copy, Clone, PartialEq)]
pub struct MoveRandomly;

#[derive(Copy, Clone, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
