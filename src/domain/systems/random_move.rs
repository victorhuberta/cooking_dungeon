use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MoveRandomly)]
pub fn random_move(world: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Point)>::query()
        .filter(component::<MoveRandomly>())
        .iter_mut(world)
        .for_each(|(entity, pos)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(0, 1),
                2 => Point::new(1, 0),
                _ => Point::new(0, -1),
            } + *pos;
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
}
