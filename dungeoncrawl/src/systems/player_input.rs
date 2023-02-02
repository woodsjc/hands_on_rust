use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Space => Point::new(0, 0),
            _ => return, // Point::new(0, 0),
        };

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_e, destination) = players
            .iter(ecs)
            .find_map(|(e, p)| Some((*e, *p + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut hit_something = false;
        let mut did_something = false;

        enemies
            .iter(ecs)
            .filter(|(_, p)| **p == destination)
            .for_each(|(e, _)| {
                hit_something = true;
                did_something = true;
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: player_e,
                        victim: *e,
                    },
                ));
            });

        if !hit_something {
            did_something = true;
            commands.push((
                (),
                WantsToMove {
                    entity: player_e,
                    destination,
                },
            ));
        }

        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_e)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = health.max.min(health.current + 1);
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
