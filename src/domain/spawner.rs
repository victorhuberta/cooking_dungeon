use crate::prelude::*;

pub fn spawn_player(world: &mut World, position: Point) {
    world.push((
        Player,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

// Spawn a random enemy in each room except the one the player is in.
pub fn spawn_random_enemies(
    world: &mut World,
    map_builder: &MapBuilder,
    rng: &mut RandomNumberGenerator,
) {
    map_builder
        .rooms()
        .iter()
        .skip(1)
        .map(Rect::center)
        .for_each(|pos| {
            spawn_enemy(
                world,
                pos,
                match rng.range(0, 4) {
                    0 => 'E',
                    1 => 'O',
                    2 => 'o',
                    _ => 'g',
                },
            );
        });
}

pub fn spawn_enemy(world: &mut World, position: Point, enemy_type: char) {
    world.push((
        Enemy,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(enemy_type),
        },
    ));
}
