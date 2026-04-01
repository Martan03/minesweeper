use termint::{
    buffer::Buffer,
    enums::Color,
    geometry::{Padding, Rect, Vec2},
    style::Style,
    widgets::{Element, LayoutNode, Widget},
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
    fn render(&self, buffer: &mut Buffer, node: &LayoutNode) {
        self.render_inner(buffer, node);

        self.content.render(buffer, &node.children[0]);
    }

    fn height(&self, size: &Vec2) -> usize {
        self.content.height(size) + self.bar_height()
    }

    fn width(&self, size: &Vec2) -> usize {
        self.content.width(size) + 7
    }

    fn children(&self) -> Vec<&Element> {
        let mut children = vec![&self.content];
        if let Some(child) = self.top_bar.as_ref() {
            children.push(child);
        }
        if let Some(child) = self.bot_bar.as_ref() {
            children.push(child);
        }
        children
    }

    fn layout(&self, node: &mut LayoutNode, area: Rect) {
        if !node.is_dirty && !node.has_dirty_child && node.area == area {
            return;
        }

        node.area = area;
        node.is_dirty = false;
        node.has_dirty_child = false;

        let crect = area.inner(self.content_padding());
        self.content.layout(&mut node.children[0], crect);

        let mut cid = 1;
        if let Some(top) = &self.top_bar {
            let tr = Rect::new(
                area.x() + 3,
                area.y() + 1,
                area.width().saturating_sub(7),
                1,
            );
            top.layout(&mut node.children[cid], tr);
            cid += 1;
        }

        if let Some(bot) = &self.bot_bar {
            let br = Rect::new(
                area.x() + 3,
                area.y() + 4 + crect.height(),
                area.width().saturating_sub(7),
                1,
            );
            bot.layout(&mut node.children[cid], br);
        }
    }
}

impl Border {
    fn render_inner(&self, buffer: &mut Buffer, node: &LayoutNode) {
        let rect = node.area;
        let (bc, ff, sn) = Self::get_colors();

        let hframe_width = rect.width().saturating_sub(7);
        let hframe = "▄".repeat(hframe_width);
        let ehframe = " ".repeat(rect.width().saturating_sub(2));

        let ffbc = Style::new().bg(ff).fg(bc);
        let bcsn = Style::new().bg(bc).fg(sn);

        let mut pos = *rect.pos();
        let end = pos.x + hframe_width + 6;

        buffer.set_str_styled(format!(" ▄▄▄{hframe}▄▄"), &pos, ffbc);
        buffer.set_str_styled("▄", &Vec2::new(end, pos.y), bcsn);

        pos.y += 1;
        let mut cache_id = 1;
        if let Some(top) = &self.top_bar {
            buffer.set_str_styled(
                &ehframe,
                &Vec2::new(pos.x + 1, pos.y),
                bcsn,
            );
            buffer[pos].bg(ff);
            buffer.set_str_styled(
                " ",
                &Vec2::new(end, pos.y),
                Style::new().bg(sn),
            );
            top.render(buffer, &node.children[cache_id]);
            cache_id += 1;
            pos.y += 1;
        }

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
        for _ in 0..node.children[0].area.height() {
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
        if let Some(bot) = &self.bot_bar {
            buffer.set_str_styled(ehframe, &Vec2::new(pos.x + 1, pos.y), bcsn);
            buffer[pos].bg(ff);
            buffer.set_str_styled(
                " ",
                &Vec2::new(end, pos.y),
                Style::new().bg(sn),
            );
            bot.render(buffer, &node.children[cache_id]);
            pos.y += 1;
        }
        buffer.set_str_styled("▄", &pos, ffbc);
        buffer.set_str_styled(
            format!("▄▄{hframe}▄▄▄"),
            &Vec2::new(pos.x + 1, pos.y),
            bcsn,
        );
        buffer.set_str_styled(" ", &Vec2::new(end, pos.y), snbc);
    }

    fn content_padding(&self) -> Padding {
        let mut padding = Padding::new(2, 4, 2, 3);
        padding.top += self.top_bar.is_some() as usize;
        padding.bottom += self.bot_bar.is_some() as usize;
        padding
    }

    fn bar_height(&self) -> usize {
        4 + self.top_bar.is_some() as usize + self.bot_bar.is_some() as usize
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
