use legion_presentation::actions::Action::*;
use legion_presentation::actions::*;
use legion_presentation::screen::init_screen;

fn main() {
    let mut screen = init_screen();
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
        screen.render()
    }
}
