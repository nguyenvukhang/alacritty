use alacritty_terminal::term::cell::Hyperlink;

use crate::config::ui_config::{Hint, HintAction};

/// Keyboard regex hint state.
pub struct HintState {
    /// Hint currently in use.
    hint: Option<Hint>,

    /// Alphabet for hint labels.
    alphabet: String,

    /// Key label for each visible match.
    labels: Vec<Vec<char>>,

    /// Keys pressed for hint selection.
    keys: Vec<char>,
}

impl HintState {
    /// Initialize an inactive hint state.
    pub fn new<S: Into<String>>(alphabet: S) -> Self {
        Self {
            alphabet: alphabet.into(),
            hint: Default::default(),
            labels: Default::default(),
            keys: Default::default(),
        }
    }

    /// Check if a hint selection is in progress.
    pub fn active(&self) -> bool {
        self.hint.is_some()
    }

    /// Start the hint selection process.
    pub fn start(&mut self, hint: Hint) {
        self.hint = Some(hint);
    }

    /// Cancel the hint highlighting process.
    fn stop(&mut self) {
        self.labels.clear();
        self.keys.clear();
        self.hint = None;
    }

    /// Handle keyboard input during hint selection.
    pub fn keyboard_input(&mut self, c: char) -> Option<HintMatch> {
        match c {
            // Use backspace to remove the last character pressed.
            '\x08' | '\x1f' => {
                self.keys.pop();
            },
            // Cancel hint highlighting on ESC/Ctrl+c.
            '\x1b' | '\x03' => self.stop(),
            _ => (),
        }

        None
    }

    /// Update the alphabet used for hint labels.
    pub fn update_alphabet(&mut self, alphabet: &str) {
        if self.alphabet != alphabet {
            self.alphabet = alphabet.to_owned();
            self.keys.clear();
        }
    }
}

/// Hint match which was selected by the user.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HintMatch {
    /// Action for handling the text.
    action: HintAction,

    hyperlink: Option<Hyperlink>,
}

impl HintMatch {
    #[inline]
    pub fn action(&self) -> &HintAction {
        &self.action
    }

    pub fn hyperlink(&self) -> Option<&Hyperlink> {
        self.hyperlink.as_ref()
    }
}
