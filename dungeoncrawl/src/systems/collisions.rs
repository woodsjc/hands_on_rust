use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_p = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    players.iter(ecs).for_each(|p| player_p = *p);

    enemies
        .iter(ecs)
        .filter(|(_, p)| **p == player_p)
        .for_each(|(e, _)| {
            commands.remove(*e);
        });
}
