use crate::game_map::{make_map, GameMap};

use legion::world::World;

pub fn init_game(screen_height: i32, screen_width: i32) -> Game {
    return Game {
        camera_height: screen_height,
        camera_width: screen_width,
        food: 100,
        map: make_map(),
        population: 0,
        wood: 100,
        world: World::default(),
    };
}

pub struct Game {
    pub map: GameMap,
    pub camera_height: i32,
    pub camera_width: i32,
    pub population: i32,
    pub wood: i32,
    pub food: i32,
    pub world: World,
}
