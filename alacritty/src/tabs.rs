//! Tab manager.

use crate::event::EventProxy;
use alacritty_terminal::event_loop::Notifier;
use alacritty_terminal::term::Term;

pub struct TabManager {
    terminals: Vec<(Term<EventProxy>, Notifier)>,
    /// Points to the index that the selected terminal (not any of the
    /// ones in `terminals`) would be inserted at.
    current_index: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self { terminals: Vec::new(), current_index: 0 }
    }
}
