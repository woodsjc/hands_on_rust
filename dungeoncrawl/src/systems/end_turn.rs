use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYatta)]
pub fn end_turn(#[resource] turn_state: &mut TurnState, ecs: &SubWorld) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let current_state = turn_state.clone();
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYatta>());
    let amulet_p = amulet.iter(ecs).nth(0).unwrap();

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|(hp, p)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if p == amulet_p {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
