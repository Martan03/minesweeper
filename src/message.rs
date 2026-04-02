use termint::prelude::Vec2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    DiffSel(usize),
    CellReveal(Vec2),
    CellFlag(Vec2),
}
