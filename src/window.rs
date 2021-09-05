use crate::actions::Action;
use crate::actions::Action::*;
use crate::component::drawable::Drawable;
use crate::component::position::Position;
use crate::game::Game;

use tcod::console::{blit, Offscreen, Root};
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::{BackgroundFlag, Color, Console, FontLayout, FontType};

pub struct Window {
    pub root: Root,
    pub world_view: Offscreen,
    pub panel_view: Offscreen,
    pub field_of_view: FovMap,
    pub pixel_height: i32,
    pub pixel_width: i32,
}

impl Window {
    pub fn clear(&mut self) {
        self.world_view.clear();
        self.panel_view.clear();
    }

    pub fn set_fullscreen(&mut self, new_state: bool) {
        self.root.set_fullscreen(new_state);
    }

    pub fn is_fullscreen(&self) -> bool {
        return self.root.is_fullscreen();
    }

    pub fn draw_entity(&mut self, pos: &Position, drawable: &Drawable) {
        self.world_view.set_default_foreground(drawable.color);
        self.world_view
            .put_char(pos.x, pos.y, drawable.char, BackgroundFlag::None);
    }

    pub fn draw_map(&mut self, game: &mut Game, position: &Position, fov_recompute: bool) {
        if fov_recompute {
            self.field_of_view.compute_fov(
                position.x,
                position.y,
                TORCH_RADIUS,
                FOV_LIGHT_WALLS,
                FOV_ALGO,
            );
        }

        let top = position.y - (game.camera_height / 2);
        let bottom = position.y + (game.camera_height / 2);
        let left = position.x - (game.camera_width / 2);
        let right = position.x + (game.camera_width / 2);

        for y in top..bottom {
            for x in left..right {
                for vertical_offset in -1..2 {
                    for horizontal_offset in -1..2 {
                        let visible = self.field_of_view.is_in_fov(x, y);
                        let mut x = x + (MAP_WIDTH * horizontal_offset);
                        x = if x < 0 {
                            0
                        } else if x >= (MAP_WIDTH * 3) {
                            (MAP_WIDTH * 3) - 1
                        } else {
                            x
                        };
                        let mut y = y + (MAP_HEIGHT * vertical_offset);
                        y = if y < 0 {
                            0
                        } else if y >= (MAP_HEIGHT * 3) {
                            (MAP_HEIGHT * 3) - 1
                        } else {
                            y
                        };
                        if visible {
                            game.map.set_tile_explored(true, x as usize, y as usize);
                        }
                    }
                }
                let visible = self.field_of_view.is_in_fov(x, y);
                let tile = game.map.get_tile(x as usize, y as usize);
                let color = if visible {
                    game.map.set_tile_explored(true, x as usize, y as usize);
                    tile.color
                } else if !tile.explored {
                    Color {
                        r: 242,
                        g: 227,
                        b: 211,
                    }
                } else {
                    Color {
                        r: tile.color.r / 3,
                        g: tile.color.g / 3,
                        b: tile.color.b / 3,
                    }
                };
                self.world_view
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    pub fn render(&mut self, position: &Position) {
        let source_x = position.x - (self.pixel_width / 2);
        let source_y = position.y - (self.pixel_height / 2);

        blit(
            &self.world_view,
            (source_x, source_y),
            (self.pixel_width, self.pixel_height),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
        self.root.flush()
    }

    pub fn is_window_open(&mut self) -> bool {
        !self.root.window_closed()
    }

    pub fn handle_keys(&mut self) -> Vec<Action> {
        use tcod::input::KeyCode::*;
        use tcod::input::*;

        let mut actions = vec![];
        let key_option = self.root.check_for_keypress(KEY_PRESSED);
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
}

pub fn init_screen() -> Window {
    tcod::system::set_fps(LIMIT_FPS);
    let (screen_width, screen_height) = tcod::system::get_current_resolution();
    let pixel_width = screen_width / 20;
    let pixel_height = screen_height / 20;

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(pixel_width, pixel_height)
        .title("Rouge Civ")
        .init();

    let screen = Window {
        root,
        world_view: Offscreen::new(MAP_WIDTH * 3, MAP_HEIGHT * 3),
        panel_view: Offscreen::new(screen_width, PANEL_HEIGHT),
        field_of_view: FovMap::new(MAP_WIDTH * 3, MAP_HEIGHT * 3),
        pixel_height,
        pixel_width,
    };

    return screen;
}

pub const MAP_WIDTH: i32 = 1000;
pub const MAP_HEIGHT: i32 = 450;

pub const LIMIT_FPS: i32 = 60;

pub const PANEL_HEIGHT: i32 = 7;

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Shadow; // default FOV algorithm
pub const FOV_LIGHT_WALLS: bool = true; // light walls or not
pub const TORCH_RADIUS: i32 = 10;
