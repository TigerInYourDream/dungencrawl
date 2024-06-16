pub use crate::prelude::*;

pub fn spwan_player(ecs: &mut World, pos: Point) -> Entity {
    ecs.push((
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Player,
    ))
}
