use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MoveRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MoveRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    movers.iter(ecs).for_each(|(entity, p, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *p;

        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, t, _)| **t == destination)
            .for_each(|(v, _, _)| {
                if ecs.entry_ref(*v).unwrap().get_component::<Player>().is_ok() {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *v,
                        },
                    ));
                }
                attacked = true;
            });

        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
