use crate::actions::Action::*;
use crate::component::building::House;
use crate::component::drawable::Drawable;
use crate::component::player::Player;
use crate::component::position::Position;
use crate::game::Game;
use crate::game_map::COLOR_VILLAGE;
use crate::window::{MAP_HEIGHT, MAP_WIDTH};

use legion::IntoQuery;
use crate::resource::inventory::{Inventory, Player_Inventory};

#[derive(PartialEq, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Build,
    FullScreen,
    Quit,
}

pub fn process_player_action(action: Action, mut game: &mut Game) {
    match action {
        MoveUp => {
            let mut query = <(&Player, &mut Position)>::query();
            let position = query.iter_mut(&mut game.world).next().unwrap().1;
            position.y -= 1;
            if position.y < MAP_HEIGHT {
                position.y = MAP_HEIGHT * 2 - 1
            }
            let (x, y) = (position.x, position.y);
            if game.map.is_tile_blocked(x, y) {
                position.y += 1;
            }
        }
        MoveDown => {
            let mut query = <(&Player, &mut Position)>::query();
            let position = query.iter_mut(&mut game.world).next().unwrap().1;
            position.y += 1;
            if position.y >= MAP_HEIGHT * 2 {
                position.y = 0 + MAP_HEIGHT
            }
            if game.map.is_tile_blocked(position.x, position.y) {
                position.y -= 1;
            }
        }
        MoveLeft => {
            let mut query = <(&Player, &mut Position)>::query();
            let position = query.iter_mut(&mut game.world).next().unwrap().1;
            position.x -= 1;
            if position.x < MAP_WIDTH {
                position.x = MAP_WIDTH * 2 - 1
            }
            let (x, y) = (position.x, position.y);
            if game.map.is_tile_blocked(x, y) {
                position.x += 1;
            }
        }
        MoveRight => {
            let mut query = <(&Player, &mut Position)>::query();
            let position = query.iter_mut(&mut game.world).next().unwrap().1;
            position.x += 1;
            if position.x >= MAP_WIDTH * 2 {
                position.x = 0 + MAP_WIDTH
            }
            if game.map.is_tile_blocked(position.x, position.y) {
                position.x -= 1;
            }
        }
        Build => {
            let mut query = <(&Player, &Position, &mut Player_Inventory)>::query();
            let player = query.iter_mut(&mut game.world).next().unwrap();
            let player_pos = Position {
                x: player.1.x,
                y: player.1.y,
            };
            if game.map.is_buildable(player_pos.x, player_pos.y) && player.2.wood >= 10 {
                player.2.wood_change -= 10;
                game.map.make_tile_built_on(player_pos.x, player_pos.y);
                game.world.push((
                    Position::new(player_pos.x, player_pos.y),
                    Drawable::new('A', COLOR_VILLAGE),
                    House::new(),
                ));
            }
        }
        _ => {}
    }
}
