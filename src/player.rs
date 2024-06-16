use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::W | VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::A | VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::S | VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::D | VirtualKeyCode::Right => Point::new(1, 0),
                _ => Point::zero(),
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
