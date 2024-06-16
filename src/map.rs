use crate::{player::Player, prelude::*, NUM_TILES};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.tiles[map_idx(x, y)] {
                    TileType::Wall => {
                        ctx.set(x, y, RGB::named(RED), RGB::named(BLACK), to_cp437('#'));
                    }
                    TileType::Floor => {
                        ctx.set(x, y, RGB::named(GREEN), RGB::named(BLACK), to_cp437('.'));
                    }
                }
            }
        }
    }

    pub fn in_bound(&self, p: Point) -> bool {
        p.x >= 0 && p.x < SCREEN_WIDTH && p.y >= 0 && p.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, p: Point) -> bool {
        self.in_bound(p) && self.tiles[map_idx(p.x, p.y)] == TileType::Floor
    }

    pub fn try_idx(&self, p: Point) -> Option<usize> {
        if !self.in_bound(p) {
            return None;
        }
        Some(map_idx(p.x, p.y))
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct State {
    player: Player,
    map: Map,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
