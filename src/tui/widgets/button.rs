use termint::{
    buffer::{Buffer, Cell},
    enums::Color,
    geometry::{Rect, Vec2},
    style::Style,
    widgets::{cache::Cache, Element, Widget},
};

#[derive(Debug)]
pub struct Button {
    content: Element,
    selected: bool,
}

impl Button {
    /// Creates new minesweeper style [`Button`]
    pub fn new<E>(content: E) -> Self
    where
        E: Into<Element>,
    {
        Self {
            content: content.into(),
            selected: false,
        }
    }

    /// Selects [`Button`]
    pub fn select(mut self) -> Self {
        self.selected = true;
        self
    }

    /// Sets whether [`Button`] is selected or not
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for Button {
    fn render(&self, buffer: &mut Buffer, rect: Rect, cache: &mut Cache) {
        let (lb, db, w) = self.get_colors();

        let crect = rect.inner((1, 1, 1, 2));
        let hline = "▄".repeat(crect.width());

        let wlb = Style::new().bg(w).fg(lb);
        let mut pos = *rect.pos();

        buffer.set_str_styled(format!("▌▗{hline}▛"), &pos, wlb);
        buffer.set_fg(db, &pos);
        buffer.set_bg(db, &Vec2::new(pos.x + crect.width() + 2, pos.y));

        pos.y += 1;
        buffer[pos] = Cell::new('▌').bg(w).fg(db);
        buffer.set_str_styled(
            format!("▌{}", " ".repeat(crect.width() + 1)),
            &Vec2::new(pos.x + 1, pos.y),
            Style::new().bg(lb).fg(w),
        );
        buffer[Vec2::new(pos.x + crect.width() + 2, pos.y)] =
            Cell::new('▌').bg(db).fg(lb);

        pos.y += 1;
        buffer[pos] = Cell::new('▌').bg(w).fg(db);
        pos.x += 1;
        buffer.set_str_styled(
            format!("▘{hline}▟"),
            &pos,
            Style::new().bg(lb).fg(db),
        );
        buffer.set_fg(w, &pos);

        self.content.render(buffer, crect, &mut cache.children[0]);
    }

    fn height(&self, _size: &Vec2) -> usize {
        3
    }

    fn width(&self, size: &Vec2) -> usize {
        self.content.width(&Vec2::new(size.x, 1)) + 3
    }

    fn children(&self) -> Vec<&Element> {
        vec![&self.content]
    }
}

impl Button {
    fn get_colors(&self) -> (Color, Color, Color) {
        if self.selected {
            (
                Color::Hex(0x999999),
                Color::Hex(0x696969),
                Color::Hex(0xcccccc),
            )
        } else {
            (
                Color::Hex(0xbcbcbc),
                Color::Hex(0x797979),
                Color::Hex(0xffffff),
            )
        }
    }
}

impl From<Button> for Box<dyn Widget> {
    fn from(value: Button) -> Self {
        Box::new(value)
    }
}

impl From<Button> for Element {
    fn from(value: Button) -> Self {
        Element::new(value)
    }
}
