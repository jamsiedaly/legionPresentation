use legion_presentation::actions::Action::*;
use legion_presentation::actions::*;
use legion_presentation::game::{init_game, Game};
use legion_presentation::position::Position;
use legion_presentation::screen::{init_screen, Screen};
use tcod::console::blit;
use tcod::{BackgroundFlag, Console};

fn main() {
    let mut screen = init_screen();
    let mut game = init_game(screen.pixel_height, screen.pixel_width);
    let position = Position::new(250, 250);
    'game_loop: while screen.is_window_open() {
        let actions = handle_keys(&screen.root);
        for action in actions {
            if action == FullScreen {
                let fullscreen = screen.root.is_fullscreen();
                screen.root.set_fullscreen(!fullscreen);
            } else if action == Quit {
                break 'game_loop;
            }
        }
        blit_to_screen(&game, &mut screen, &position);
        screen.render(&position);
    }
}

pub fn blit_to_screen(game: &Game, screen: &mut Screen, position: &Position) {
    let top = position.y - (game.camera_height / 2);
    let bottom = position.y + (game.camera_height / 2);
    let left = position.x - (game.camera_width / 2);
    let right = position.x + (game.camera_width / 2);

    for y in top..bottom {
        for x in left..right {
            let tile = game.map.get_tile(x as usize, y as usize);
            let color = tile.color;
            screen
                .world
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }
}
