use legion_presentation::actions::Action::*;
use legion_presentation::actions::*;
use legion_presentation::component::drawable::Drawable;
use legion_presentation::component::player::{init_player, Player};
use legion_presentation::component::position::Position;
use legion_presentation::game::init_game;
use legion_presentation::window::{init_screen, MAP_HEIGHT, MAP_WIDTH};
use legion_presentation::resource::inventory::{Inventory, Player_Inventory};
use legion_presentation::system::systems::sync_resources_system;

use legion::*;
use game_time::GameClock;
use rand::rngs::ThreadRng;

fn main() {
    let mut window = init_screen();
    let mut game = init_game(window.pixel_height, window.pixel_width);

    for y in 0..MAP_HEIGHT * 3 {
        for x in 0..MAP_WIDTH * 3 {
            window.field_of_view.set(
                x,
                y,
                !game.map.is_tile_blocking_vision(x as usize, y as usize),
                !game.map.is_tile_blocked(x, y),
            );
        }
    }

    let player = init_player(&mut game);
    let previous_player_position = (-1, -1);

    let mut schedule = Schedule::builder()
        .add_system(sync_resources_system())
        .build();

    let inventory = Inventory::new(100, 100);
    let game_clock = GameClock::default();
    let mut rng = rand::thread_rng();
    let mut resources = Resources::default();
    resources.insert(inventory);
    resources.insert(game_clock);
    resources.insert(rng);

    'game_loop: loop {
        window.clear();
        let player_position = *game
            .world
            .entry(player)
            .expect("Player got lost in the ECS")
            .get_component::<Position>()
            .expect("Player has no position component");
        let fov_recompute = previous_player_position != (player_position.x, player_position.y);
        let actions = window.handle_keys();
        for action in actions {
            if action == FullScreen {
                let fullscreen = window.is_fullscreen();
                window.set_fullscreen(!fullscreen);
            } else if action == Quit {
                break 'game_loop;
            } else {
                process_player_action(action, &mut game)
            }
        }
        schedule.execute(&mut game.world, &mut resources);
        let mut query = <(&Drawable, &Position)>::query();

        window.draw_map(&mut game, &player_position, fov_recompute);
        for (drawable, position) in query.iter(&game.world) {
            window.draw_entity(position, drawable);
        }
        window.render(&player_position);
    }
}