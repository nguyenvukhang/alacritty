//! Tab manager.

use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Arc;

use crate::event::EventProxy;
use alacritty_terminal::event_loop::Notifier;
use alacritty_terminal::sync::FairMutex;
use alacritty_terminal::term::Term;

struct HiddenTerminal {
    terminal: Arc<FairMutex<Term<EventProxy>>>,
    notifier: Notifier,
    #[cfg(not(windows))]
    master_fd: RawFd,
    #[cfg(not(windows))]
    shell_pid: u32,
}

/// Virtual Tab Manager. Manages virtual tabs.
pub struct TabManager {
    /// The hidden terminals.
    hidden: Vec<HiddenTerminal>,
    /// Points to the index that the selected terminal (not any of the
    /// ones in `hidden`) would be inserted at.
    current_index: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self { hidden: Vec::new(), current_index: 0 }
    }

    /// Adds a new tab to the right of the current tab.
    pub fn add(
        &mut self,
        terminal: Arc<FairMutex<Term<EventProxy>>>,
        notifier: Notifier,
        #[cfg(not(windows))] master_fd: RawFd,
        #[cfg(not(windows))] shell_pid: u32,
    ) {
        let hidden_terminal = HiddenTerminal {
            terminal,
            notifier,
            #[cfg(not(windows))]
            master_fd,
            #[cfg(not(windows))]
            shell_pid,
        };
        self.hidden.insert(self.current_index, hidden_terminal);
    }
}
