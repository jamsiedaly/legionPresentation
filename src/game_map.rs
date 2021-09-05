use crate::window::{MAP_HEIGHT, MAP_WIDTH};

use noise::{NoiseFn, Perlin};
use tcod::Color;

pub struct GameMap {
    tiles: Vec<Vec<Tile>>,
}

impl GameMap {
    pub fn new(tiles: Vec<Vec<Tile>>) -> GameMap {
        return GameMap { tiles };
    }

    pub fn is_tile_blocked(&self, x: i32, y: i32) -> bool {
        return self.tiles[x as usize][y as usize].blocked;
    }

    pub fn is_buildable(&self, x: i32, y: i32) -> bool {
        return self.tiles[x as usize][y as usize].buildable;
    }

    pub fn make_tile_built_on(&mut self, x: i32, y: i32) {
        self.tiles[x as usize][y as usize].blocked = true;
        self.tiles[x as usize][y as usize].buildable = false;
    }

    pub fn is_tile_blocking_vision(&self, x: usize, y: usize) -> bool {
        return self.tiles[x][y].block_sight;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        return self.tiles[x][y];
    }

    pub fn set_tile_explored(&mut self, explored: bool, x: usize, y: usize) {
        self.tiles[x][y].explored = explored;
    }

    pub fn harvest(&mut self, x: i32, y: i32) -> i32 {
        return self.tiles[x as usize][y as usize].harvest();
    }
}

pub fn make_map() -> GameMap {
    let mut tiles = vec![vec![Tile::meadow(); (MAP_HEIGHT * 3) as usize]; (MAP_WIDTH * 3) as usize];
    let perlin = Perlin::new();
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let mut height = 0.0;
            let fertility = perlin.get([x as f64 / 10f64, y as f64 / 10f64, 1.999_282_82]);
            let mountain_modifier = perlin.get([x as f64 / 10f64, y as f64 / 10f64, 2.5]);
            height += perlin.get([x as f64 / 10f64, y as f64 / 10f64, GAME_SEED]);
            height += perlin.get([x as f64, y as f64, GAME_SEED + 1.0]) / 7.5;

            if height >= -0.1 {
                height += mountain_modifier.abs()
            }

            for i in 0..3 {
                for j in 0..3 {
                    if height >= 1.1 {
                        tiles[(x + (MAP_WIDTH * i)) as usize][(y + (MAP_HEIGHT * j)) as usize] =
                            Tile::mountain()
                    } else if height >= 0.50 {
                        tiles[(x + (MAP_WIDTH * i)) as usize][(y + (MAP_HEIGHT * j)) as usize] =
                            Tile::hill()
                    } else if height < -0.175 {
                        tiles[(x + (MAP_WIDTH * i)) as usize][(y + (MAP_HEIGHT * j)) as usize] =
                            Tile::water()
                    } else if fertility >= 0.25 {
                        tiles[(x + (MAP_WIDTH * i)) as usize][(y + (MAP_HEIGHT * j)) as usize] =
                            Tile::forest()
                    }
                }
            }
        }
    }
    GameMap::new(tiles)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile {
    blocked: bool,
    block_sight: bool,
    pub explored: bool,
    buildable: bool,
    pub color: Color,
    pub fertility: i32,
}

impl Tile {
    pub fn is_blocked(&self) -> bool {
        return self.blocked;
    }

    pub fn harvest(&mut self) -> i32 {
        if self.color == COLOR_PLAINS {
            self.color = COLOR_FARM;
        }
        return self.fertility;
    }

    pub fn meadow() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
            buildable: true,
            color: COLOR_PLAINS,
            fertility: 3,
        }
    }

    pub fn mountain() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
            buildable: false,
            color: COLOR_MOUNTAIN,
            fertility: 0,
        }
    }

    pub fn hill() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
            buildable: false,
            color: COLOR_HILL,
            fertility: 1,
        }
    }

    pub fn forest() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
            buildable: true,
            color: COLOR_FOREST,
            fertility: 1,
        }
    }

    pub fn water() -> Self {
        Tile {
            blocked: true,
            block_sight: false,
            explored: false,
            buildable: false,
            color: COLOR_SEA,
            fertility: 3,
        }
    }
}

pub const COLOR_MOUNTAIN: Color = Color {
    r: 244,
    g: 251,
    b: 252,
};
pub const COLOR_HILL: Color = Color {
    r: 214,
    g: 163,
    b: 110,
};
pub const COLOR_SEA: Color = Color {
    r: 127,
    g: 191,
    b: 191,
};
pub const COLOR_FOREST: Color = Color {
    r: 127,
    g: 191,
    b: 127,
};
pub const COLOR_PLAINS: Color = Color {
    r: 161,
    g: 214,
    b: 110,
};
pub const COLOR_FARM: Color = Color {
    r: 201,
    g: 219,
    b: 0,
};
pub const COLOR_VILLAGE: Color = Color {
    r: 161,
    g: 144,
    b: 110,
};
pub const COLOR_PERSON: Color = Color {
    r: 255,
    g: 255,
    b: 153,
};

pub const GAME_SEED: f64 = 1.5;
