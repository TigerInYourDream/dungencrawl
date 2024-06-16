use crate::prelude::*;

mod entry_render;
mod map_render;
mod player_input;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entry_render::entity_render_system())
        .build()
}