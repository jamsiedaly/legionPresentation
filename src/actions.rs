use crate::actions::Action::*;
use tcod::console::Root;

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

pub fn handle_keys(root: &Root) -> Vec<Action> {
    use tcod::input::KeyCode::*;
    use tcod::input::*;

    let mut actions = vec![];
    let key_option = root.check_for_keypress(KEY_PRESSED);
    match key_option {
        Some(key) => match key {
            Key {
                code: Enter,
                alt: true,
                ..
            } => actions.push(FullScreen),
            Key { code: Escape, .. } => actions.push(Quit),
            Key { code: Up, .. } => actions.push(MoveUp),
            Key { code: Down, .. } => actions.push(MoveDown),
            Key { code: Left, .. } => actions.push(MoveLeft),
            Key { code: Right, .. } => actions.push(MoveRight),
            Key { code: Spacebar, .. } => actions.push(Build),
            _ => {}
        },
        _ => {}
    };
    return actions;
}
