use termint::{
    buffer::Buffer,
    enums::Color,
    geometry::Vec2,
    style::Style,
    widgets::{Element, Widget},
};

pub struct Border<T = Element> {
    content: T,
    top_bar: Option<Element>,
    bot_bar: Option<Element>,
    bg: bool,
}

impl<T> Border<T>
where
    T: Widget,
{
    pub fn new(content: T, bg: bool) -> Self {
        Self {
            content,
            top_bar: None,
            bot_bar: None,
            bg,
        }
    }

    /// Sets top bar to given [`Widget`]
    pub fn top_bar<E>(mut self, bar: E) -> Self
    where
        E: Into<Element>,
    {
        self.top_bar = Some(bar.into());
        self
    }

    /// Sets bottom bar to given [`Widget`]
    pub fn bot_bar<E>(mut self, bar: E) -> Self
    where
        E: Into<Element>,
    {
        self.bot_bar = Some(bar.into());
        self
    }
}

impl<T> Widget for Border<T>
where
    T: Widget,
{
    fn render(&self, buffer: &mut Buffer) {
        self.render_inner(buffer);

        let mut cbuffer = buffer.subset(buffer.rect().inner((2, 4, 2, 3)));
        self.content.render(&mut cbuffer);
        buffer.merge(cbuffer);
    }

    fn height(&self, size: &Vec2) -> usize {
        self.content.height(size) + 4
        // + self.top_bar.is_some() as usize
        // + self.bot_bar.is_some() as usize
    }

    fn width(&self, size: &Vec2) -> usize {
        self.content.width(size) + 7
    }
}

impl<T> Border<T>
where
    T: Widget,
{
    fn render_inner(&self, buffer: &mut Buffer) {
        let (bc, ff, sn) = Self::get_colors();

        let hframe_width = buffer.width().saturating_sub(7);
        let hframe = "▄".repeat(hframe_width);

        let ffbc = Style::new().bg(ff).fg(bc);
        let bcsn = Style::new().bg(bc).fg(sn);

        let mut pos = *buffer.pos();
        let end = pos.x + hframe_width + 6;

        buffer.set_str_styled(format!(" ▄▄▄{hframe}▄▄"), &pos, ffbc);
        buffer.set_str_styled("▄", &Vec2::new(end, pos.y), bcsn);

        pos.y += 1;
        buffer.set_str_styled(" ", &pos, ffbc);
        buffer.set_str_styled(
            format!(" ▗▄{hframe}  "),
            &Vec2::new(pos.x + 1, pos.y),
            bcsn,
        );
        buffer.set_str_styled(
            " ",
            &Vec2::new(pos.x + hframe_width + 6, pos.y),
            Style::new().bg(sn),
        );

        let snbc = Style::new().bg(sn).fg(bc);
        let bcff = Style::new().bg(bc).fg(ff);

        let mut sel = Style::new().bg(ff).fg(sn);
        if self.bg {
            sel = ffbc;
        }

        let bgframe = " ".repeat(hframe_width);
        for _ in 0..buffer.height().saturating_sub(4) {
            pos.y += 1;

            buffer.set_str_styled(" ", &pos, ffbc);
            buffer.set_str_styled(" ", &Vec2::new(pos.x + 1, pos.y), bcff);
            buffer.set_str_styled("▌", &Vec2::new(pos.x + 2, pos.y), snbc);
            buffer.set_str_styled(
                format!("▌{bgframe}"),
                &Vec2::new(pos.x + 3, pos.y),
                bcsn,
            );

            pos.x += hframe_width + 3;
            buffer.set_str_styled("▌", &pos, sel);
            buffer.set_str_styled("▌ ", &Vec2::new(pos.x + 1, pos.y), bcff);
            buffer.set_str_styled(" ", &Vec2::new(pos.x + 3, pos.y), snbc);

            pos.x = buffer.x();
        }

        pos.y += 1;
        buffer.set_str_styled(" ", &pos, ffbc);
        buffer.set_str_styled("  ", &Vec2::new(pos.x + 1, pos.y), bcff);
        buffer.set_str_styled(
            format!("{hframe}▄▟"),
            &Vec2::new(pos.x + 3, pos.y),
            ffbc,
        );
        buffer.set_str_styled(" ", &Vec2::new(end - 1, pos.y), bcff);
        buffer.set_str_styled(" ", &Vec2::new(end, pos.y), snbc);

        pos.y += 1;
        buffer.set_str_styled("▄", &pos, ffbc);
        buffer.set_str_styled(
            format!("▄▄{hframe}▄▄▄"),
            &Vec2::new(pos.x + 1, pos.y),
            bcsn,
        );
        buffer.set_str_styled(" ", &Vec2::new(end, pos.y), snbc);
    }

    fn get_colors() -> (Color, Color, Color) {
        (
            Color::Hex(0xbcbcbc),
            Color::Hex(0xffffff),
            Color::Hex(0x797979),
        )
    }
}

impl<T> From<Border<T>> for Element
where
    T: Widget + 'static,
{
    fn from(value: Border<T>) -> Self {
        Element::new(value)
    }
}

impl<T> From<Border<T>> for Box<dyn Widget>
where
    T: Widget + 'static,
{
    fn from(value: Border<T>) -> Self {
        Box::new(value)
    }
}
