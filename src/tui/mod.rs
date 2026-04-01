pub mod diff_picker;
pub mod game;
pub mod help;
pub mod widgets;

// pub use widgets::raw_span;

use crate::message::Message;

pub type Element = termint::widgets::Element<Message>;
