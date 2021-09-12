use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
pub fn collisions(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(world)
        .next();
    if let Some(player_pos) = player_pos {
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        for (enemy, enemy_pos) in enemies.iter(world) {
            if *enemy_pos == *player_pos {
                commands.remove(*enemy);
            }
        }
    }
}
