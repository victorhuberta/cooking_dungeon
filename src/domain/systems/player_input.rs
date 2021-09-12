use crate::prelude::*;

#[system]
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(
    world: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
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
            let mut players = <&mut Point>::query().filter(component::<Player>());
            if let Some(player_pos) = players.iter_mut(world).next() {
                let destination = *player_pos + delta;
                if map.can_enter_tile(destination) {
                    *player_pos = destination;
                    camera.on_player_move(destination);
                }
            }
        }
    }
}
