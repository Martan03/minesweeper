use termint::{
    buffer::Buffer,
    enums::{Color, Modifier},
    geometry::{Rect, Vec2},
    style::Style,
    widgets::{cache::Cache, Element, Widget},
};

/// Widget that prints text on given coordinates
///
/// It doesn't implement any wrapping or anything else, it is used only for
/// raw printing - for example using len() on emojis returns 4 and Span widget
/// adds ellipsis and doesn't print the emoji, when the width is less then 4
#[derive(Debug, Clone)]
pub struct RawSpan {
    text: String,
    style: Style,
}

impl RawSpan {
    /// Creates new raw span
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
            style: Default::default(),
        }
    }

    /// Sets style of the [`RawSpan`]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets foreground color of [`RawSpan`]
    pub fn fg<T>(mut self, fg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.fg(fg);
        self
    }

    /// Sets background color of [`RawSpan`]
    pub fn bg<T>(mut self, bg: T) -> Self
    where
        T: Into<Option<Color>>,
    {
        self.style = self.style.bg(bg);
        self
    }

    /// Sets [`RawSpan`] modifier
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.style = self.style.modifier(modifier);
        self
    }
}

impl Widget for RawSpan {
    fn render(&self, buffer: &mut Buffer, rect: Rect, _cache: &mut Cache) {
        let stext: String = self.text.chars().take(buffer.area()).collect();
        buffer.set_str_styled(&stext, rect.pos(), self.style);
    }

    fn height(&self, size: &Vec2) -> usize {
        let mut height = 0;
        for line in self.text.lines() {
            height +=
                (line.chars().count() as f32 / size.x as f32).ceil() as usize;
        }
        height
    }

    fn width(&self, size: &Vec2) -> usize {
        let mut width = 0;
        for line in self.text.lines() {
            let w =
                (line.chars().count() as f32 / size.x as f32).ceil() as usize;
            width = width.max(w);
        }
        width
    }
}

impl From<RawSpan> for Element {
    fn from(value: RawSpan) -> Self {
        Element::new(value)
    }
}

impl From<RawSpan> for Box<dyn Widget> {
    fn from(value: RawSpan) -> Self {
        Box::new(value)
    }
}
