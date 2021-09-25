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

#[derive(Copy, Clone, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
