use tcod::{BackgroundFlag, Color, Console};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drawable {
    pub char: char,
    pub color: Color,
}

impl Drawable {
    pub fn new(char: char, color: Color) -> Drawable {
        return Drawable { char, color };
    }
    pub fn draw(&self, canvas: &mut dyn Console, x: i32, y: i32) {
        canvas.set_default_foreground(self.color);
        canvas.put_char(x, y, self.char, BackgroundFlag::None);
    }
}
