use termint::{
    buffer::{Buffer, Cell},
    enums::Color,
    geometry::{Rect, Vec2},
    style::Style,
    widgets::{Element, LayoutNode, Widget},
};

#[derive(Debug)]
pub struct Button<M: 'static> {
    content: Element<M>,
    selected: bool,
}

impl<M: Clone + 'static> Button<M> {
    /// Creates new minesweeper style [`Button`]
    pub fn new<E>(content: E) -> Self
    where
        E: Into<Element<M>>,
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

impl<M: Clone + 'static> Widget<M> for Button<M> {
    fn render(&self, buffer: &mut Buffer, node: &LayoutNode) {
        let rect = node.area;
        let (lb, db, w) = self.get_colors();

        let crect = node.children[0].area;
        let hline = "▄".repeat(crect.width());

        let wlb = Style::new().bg(w).fg(lb);
        let mut pos = *rect.pos();

        buffer.set_str_styled(format!("▌▗{hline}▛"), &pos, wlb);
        buffer.set_fg(db, &pos);
        buffer.set_bg(db, &Vec2::new(pos.x + crect.width() + 2, pos.y));

        pos.y += 1;
        let mut cell_wdb = Cell::new("▌");
        cell_wdb.bg(w).fg(db);
        buffer[pos] = cell_wdb.clone();

        buffer.set_str_styled(
            format!("▌{}", " ".repeat(crect.width() + 1)),
            &Vec2::new(pos.x + 1, pos.y),
            Style::new().bg(lb).fg(w),
        );

        let mut cell_dblb = Cell::new("▌");
        cell_dblb.bg(db).fg(lb);
        buffer[Vec2::new(pos.x + crect.width() + 2, pos.y)] = cell_dblb;

        pos.y += 1;
        buffer[pos] = cell_wdb;
        pos.x += 1;
        buffer.set_str_styled(
            format!("▘{hline}▟"),
            &pos,
            Style::new().bg(lb).fg(db),
        );
        buffer.set_fg(w, &pos);

        self.content.render(buffer, &node.children[0]);
    }

    fn height(&self, _size: &Vec2) -> usize {
        3
    }

    fn width(&self, size: &Vec2) -> usize {
        self.content.width(&Vec2::new(size.x, 1)) + 3
    }

    fn children(&self) -> Vec<&Element<M>> {
        vec![&self.content]
    }

    fn layout(&self, node: &mut LayoutNode, area: Rect) {
        if !node.is_dirty && !node.has_dirty_child && node.area == area {
            return;
        }

        node.area = area;
        node.is_dirty = false;
        node.has_dirty_child = false;

        let crect = area.inner((1, 1, 1, 2));
        self.content.layout(&mut node.children[0], crect);
    }
}

impl<M: Clone + 'static> Button<M> {
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

impl<M: Clone + 'static> From<Button<M>> for Box<dyn Widget<M>> {
    fn from(value: Button<M>) -> Self {
        Box::new(value)
    }
}

impl<M: Clone + 'static> From<Button<M>> for Element<M> {
    fn from(value: Button<M>) -> Self {
        Element::new(value)
    }
}
