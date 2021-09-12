use crate::prelude::*;

#[system]
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            if let Some((player, player_pos)) = players.iter(world).next() {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *player,
                        destination: *player_pos + delta,
                    },
                ));
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
