use crate::component::drawable::Drawable;
use crate::component::position::Position;
use crate::game::Game;
use crate::game_map::{GameMap, Tile};
use crate::resource::inventory::PlayerInventory;
use crate::window::{MAP_HEIGHT, MAP_WIDTH};
use legion::Entity;
use rand::Rng;
use tcod::colors::WHITE;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub alive: bool,
}

impl Player {
    pub fn new(alive: bool) -> Player {
        return Player { alive };
    }
}

pub fn init_player(game: &mut Game) -> Entity {
    return loop {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..MAP_WIDTH);
        let y = rng.gen_range(0..MAP_HEIGHT);
        if surrounded_by_land(x, y, &game.map) {
            break game.world.push((
                Position::new(x + MAP_WIDTH, y + MAP_HEIGHT),
                Drawable::new('@', WHITE),
                Player::new(true),
                PlayerInventory::new(100, 100),
            ));
        }
    };
}

fn surrounded_by_land(x: i32, y: i32, map: &GameMap) -> bool {
    for x_offset in -1..1 {
        for y_offset in -1..1 {
            let tile: Tile =
                map.get_tile((x + x_offset).abs() as usize, (y + y_offset).abs() as usize);
            return !tile.is_blocked();
        }
    }
    return true;
}
