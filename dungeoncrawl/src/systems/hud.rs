use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health = <&Health>::query().filter(component::<Player>());
    let player_health = health.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(3);
    draw_batch.print_centered(1, "Explore!!! Use cursor keys to move");
    draw_batch.bar_horizontal(
        Point::zero(),
        DISPLAY_WIDTH,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.submit(10000).expect("Health batch error");
}
