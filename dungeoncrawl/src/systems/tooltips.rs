use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos - offset;
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(3);
    positions
        .iter(ecs)
        .filter(|(_, p, _)| **p == map_pos)
        .for_each(|(e, _, n)| {
            let screen_pos = *mouse_pos; // + Point::new(1, 1); // * 4;
            let display = if let Ok(health) = ecs.entry_ref(*e).unwrap().get_component::<Health>() {
                format!("{}: {} hp", &n.0, health.current)
            } else {
                n.0.clone()
            };
            draw_batch.print(screen_pos, &display);
            //println!("screen_pos: {:?}, display: {:?}", screen_pos, display);
        });
    draw_batch.submit(10100).expect("tooltip batch error");
}
