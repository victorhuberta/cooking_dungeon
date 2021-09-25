use crate::prelude::*;

pub fn spawn_player(world: &mut World, position: Point) {
    world.push((
        Player,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
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
            let (hp, name, enemy_type) = match rng.roll_dice(1, 10) {
                1..=8 => goblin(),
                _ => orc(),
            };
            spawn_enemy(world, pos, hp, name, enemy_type);
        });
}

pub fn spawn_enemy(world: &mut World, position: Point, hp: i32, name: String, enemy_type: char) {
    world.push((
        Enemy,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(enemy_type),
        },
        MoveRandomly,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, char) {
    (1, "Goblin".to_string(), 'g')
}

fn orc() -> (i32, String, char) {
    (2, "Orc".to_string(), 'o')
}
