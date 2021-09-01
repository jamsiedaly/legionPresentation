use tcod::console::{blit, Offscreen, Root};
use tcod::map::Map as FovMap;
use tcod::{FontLayout, FontType};
use crate::position::Position;

pub struct Screen {
    pub root: Root,
    pub world: Offscreen,
    pub panel: Offscreen,
    pub field_of_view: FovMap,
    pub pixel_height: i32,
    pub pixel_width: i32,
}

impl Screen {
    pub fn render(&mut self, position: &Position) {
        let source_x = position.x - (self.pixel_width / 2);
        let source_y = position.y - (self.pixel_height / 2);

        blit(
            &self.world,
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
}

pub fn init_screen() -> Screen {
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

    let screen = Screen {
        root,
        world: Offscreen::new(MAP_WIDTH * 3, MAP_HEIGHT * 3),
        panel: Offscreen::new(screen_width, PANEL_HEIGHT),
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
