use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();
    let player_p = player.iter(ecs).nth(0).unwrap().0;
    let player_i = map_idx(player_p.x, player_p.y);
    let search_targets = vec![player_i];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(e, p, _)| {
        let idx = map_idx(p.x, p.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*p, *player_p);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_p
            };
            let mut attacked = false;

            positions
                .iter(ecs)
                .filter(|(_, target_p, _)| **target_p == destination)
                .for_each(|(v, _, _)| {
                    if ecs.entry_ref(*v).unwrap().get_component::<Player>().is_ok() {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *e,
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
                        entity: *e,
                        destination,
                    },
                ));
            }
        }
    });
}
