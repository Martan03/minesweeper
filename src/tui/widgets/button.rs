use termint::{
    buffer::{Buffer, Cell},
    enums::Color,
    geometry::Vec2,
    style::Style,
    widgets::{Element, Widget},
};

pub struct Button<T = Element> {
    content: T,
    selected: bool,
}

impl<T> Button<T>
where
    T: Widget,
{
    /// Creates new minesweeper style [`Button`]
    pub fn new(content: T) -> Self {
        Self {
            content: content,
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

impl<T> Widget for Button<T>
where
    T: Widget,
{
    fn render(&self, buffer: &mut Buffer) {
        let (lb, db, w) = self.get_colors();

        let crect = buffer.rect().inner((1, 1, 1, 2));
        let hline = "▄".repeat(crect.width());

        let wlb = Style::new().bg(w).fg(lb);
        let mut pos = *buffer.pos();

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

        let mut cbuffer = buffer.subset(crect);
        self.content.render(&mut cbuffer);
        buffer.merge(cbuffer);
    }

    fn height(&self, _size: &Vec2) -> usize {
        3
    }

    fn width(&self, size: &Vec2) -> usize {
        self.content.width(&Vec2::new(size.x, 1)) + 3
    }
}

impl<T> Button<T>
where
    T: Widget,
{
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

impl<W> From<Button<W>> for Box<dyn Widget>
where
    W: Widget + 'static,
{
    fn from(value: Button<W>) -> Self {
        Box::new(value)
    }
}

impl<W> From<Button<W>> for Element
where
    W: Widget + 'static,
{
    fn from(value: Button<W>) -> Self {
        Element::new(value)
    }
}
