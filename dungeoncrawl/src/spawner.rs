use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, point: Point) {
    ecs.push((
        Player,
        point,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, point: Point) {
    ecs.push((
        Enemy,
        point,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
        MoveRandomly,
    ));
}
