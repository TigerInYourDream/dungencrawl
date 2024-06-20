use crate::prelude::*;

mod collisions;
mod entry_render;
mod map_render;
mod player_input;
mod random_move;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entry_render::entity_render_system())
        .add_system(random_move::move_random_system())
        .build()
}
