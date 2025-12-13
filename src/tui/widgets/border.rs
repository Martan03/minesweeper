use termint::{
    buffer::Buffer,
    enums::Color,
    geometry::{Rect, Vec2},
    style::Style,
    widgets::{cache::Cache, Element, Widget},
};

pub struct Border {
    content: Element,
    top_bar: Option<Element>,
    bot_bar: Option<Element>,
    bg: bool,
}

impl Border {
    pub fn new<E>(content: E, bg: bool) -> Self
    where
        E: Into<Element>,
    {
        Self {
            content: content.into(),
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

impl Widget for Border {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        self.render_inner(buffer, rect);

        let crect = rect.inner((2, 4, 2, 3));
        self.content.render(buffer, crect, &mut cache.children[0]);
    }

    fn height(&self, size: &Vec2) -> usize {
        self.content.height(size) + 4
        // + self.top_bar.is_some() as usize
        // + self.bot_bar.is_some() as usize
    }

    fn width(&self, size: &Vec2) -> usize {
        self.content.width(size) + 7
    }

    fn children(&self) -> Vec<&Element> {
        vec![&self.content]
        // if let Some(child) = self.top_bar.as_ref() {
        //     children.push(child);
        // }
        // if let Some(child) = self.bot_bar.as_ref() {
        //     children.push(child);
        // }
    }
}

impl Border {
    fn render_inner(&self, buffer: &mut Buffer, rect: Rect) {
        let (bc, ff, sn) = Self::get_colors();

        let hframe_width = rect.width().saturating_sub(7);
        let hframe = "▄".repeat(hframe_width);

        let ffbc = Style::new().bg(ff).fg(bc);
        let bcsn = Style::new().bg(bc).fg(sn);

        let mut pos = *rect.pos();
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
        for _ in 0..rect.height().saturating_sub(4) {
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

            pos.x = rect.x();
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

impl From<Border> for Element {
    fn from(value: Border) -> Self {
        Element::new(value)
    }
}

impl From<Border> for Box<dyn Widget> {
    fn from(value: Border) -> Self {
        Box::new(value)
    }
}
