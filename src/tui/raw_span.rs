use termint::{
    enums::{cursor::Cursor, fg::Fg},
    geometry::coords::Coords,
    widgets::widget::Widget,
};

/// Widget that prints text on given coordinates
///
/// It doesn't implement any wrapping or anything else, it is used only for
/// raw printing - for example using len() on emojis returns 4 and Span widget
/// adds ellipsis and doesn't print the emoji, when the width is less then 4
pub struct RawSpan {
    text: String,
    fg: Fg,
}

impl RawSpan {
    /// Creates new raw span
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
            fg: Default::default(),
        }
    }

    /// Sets foreground color of [`RawSpan`]
    pub fn fg(mut self, fg: Fg) -> Self {
        self.fg = fg;
        self
    }
}

impl Widget for RawSpan {
    fn render(&self, pos: &Coords, _size: &Coords) {
        print!("{}{}{}", Cursor::Pos(pos.x, pos.y), self.fg, self.text);
    }

    fn height(&self, _size: &Coords) -> usize {
        1
    }

    fn width(&self, _size: &Coords) -> usize {
        self.text.chars().count()
    }
}

impl From<RawSpan> for Box<dyn Widget> {
    fn from(value: RawSpan) -> Self {
        Box::new(value)
    }
}
