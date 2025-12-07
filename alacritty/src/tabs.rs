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

    pub fn select_next_virtual_tab(
        &mut self,
        terminal: &mut Arc<FairMutex<Term<EventProxy>>>,
        notifier: &mut Notifier,
        #[cfg(not(windows))] master_fd: &mut RawFd,
        #[cfg(not(windows))] shell_pid: &mut u32,
    ) {
        let Some(target) = self.hidden.get_mut(self.current_index) else { return };
        core::mem::swap(terminal, &mut target.terminal);
        core::mem::swap(notifier, &mut target.notifier);
        #[cfg(not(windows))]
        core::mem::swap(master_fd, &mut target.master_fd);
        #[cfg(not(windows))]
        core::mem::swap(shell_pid, &mut target.shell_pid);
    }

    pub fn select_previous_virtual_tab(
        &mut self,
        terminal: &mut Arc<FairMutex<Term<EventProxy>>>,
        notifier: &mut Notifier,
        #[cfg(not(windows))] master_fd: &mut RawFd,
        #[cfg(not(windows))] shell_pid: &mut u32,
    ) {
        if self.hidden.is_empty() {
            return;
        }
        let (i, n) = (self.current_index, self.hidden.len());
        let Some(target) = self.hidden.get_mut((i + n - 1) % n) else { return };
        core::mem::swap(terminal, &mut target.terminal);
        core::mem::swap(notifier, &mut target.notifier);
        #[cfg(not(windows))]
        core::mem::swap(master_fd, &mut target.master_fd);
        #[cfg(not(windows))]
        core::mem::swap(shell_pid, &mut target.shell_pid);
    }
}
