use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers.iter(ecs).map(|(e, a)| (*e, a.victim)).collect();

    victims.iter().for_each(|(m, v)| {
        if let Ok(mut health) = ecs.entry_mut(*v).unwrap().get_component_mut::<Health>() {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*v);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*m);
    });
}
