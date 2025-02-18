use std::path::PathBuf;

use log::error;
use serde::{self, Deserialize, Deserializer};

use alacritty_config_derive::{ConfigDeserialize, SerdeReplace};
use alacritty_terminal::config::{Config as TerminalConfig, Percentage, LOG_TARGET_CONFIG};

use crate::config::bindings::{self, Binding, KeyBinding, MouseBinding};
use crate::config::color::Colors;
use crate::config::debug::Debug;
use crate::config::font::Font;
use crate::config::mouse::Mouse;
use crate::config::window::WindowConfig;

#[derive(ConfigDeserialize, Clone, Debug, PartialEq)]
pub struct UiConfig {
    /// Font configuration.
    pub font: Font,

    /// Window configuration.
    pub window: WindowConfig,

    pub mouse: Mouse,

    /// Debug options.
    pub debug: Debug,

    /// Send escape sequences using the alt key.
    #[config(removed = "It's now always set to 'true'. If you're on macOS use \
                        'window.option_as_alt' to alter behavior of Option")]
    pub alt_send_esc: Option<bool>,

    /// Live config reload.
    pub live_config_reload: bool,

    /// RGB values for colors.
    pub colors: Colors,

    /// Should draw bold text with brighter colors instead of bold font.
    pub draw_bold_text_with_bright_colors: bool,

    /// Path where config was loaded from.
    #[config(skip)]
    pub config_paths: Vec<PathBuf>,

    /// Offer IPC through a unix socket.
    #[cfg(unix)]
    pub ipc_socket: bool,

    /// Config for the alacritty_terminal itself.
    #[config(flatten)]
    pub terminal_config: TerminalConfig,

    /// Keybindings.
    key_bindings: KeyBindings,

    /// Bindings for the mouse.
    mouse_bindings: MouseBindings,

    /// Background opacity from 0.0 to 1.0.
    #[config(deprecated = "use window.opacity instead")]
    background_opacity: Option<Percentage>,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            live_config_reload: true,
            alt_send_esc: Default::default(),
            #[cfg(unix)]
            ipc_socket: true,
            font: Default::default(),
            window: Default::default(),
            mouse: Default::default(),
            debug: Default::default(),
            config_paths: Default::default(),
            key_bindings: Default::default(),
            mouse_bindings: Default::default(),
            terminal_config: Default::default(),
            background_opacity: Default::default(),
            colors: Default::default(),
            draw_bold_text_with_bright_colors: Default::default(),
        }
    }
}

impl UiConfig {
    #[inline]
    pub fn window_opacity(&self) -> f32 {
        self.background_opacity.unwrap_or(self.window.opacity).as_f32()
    }

    #[inline]
    pub fn key_bindings(&self) -> &[KeyBinding] {
        self.key_bindings.0.as_slice()
    }

    #[inline]
    pub fn mouse_bindings(&self) -> &[MouseBinding] {
        self.mouse_bindings.0.as_slice()
    }
}

#[derive(SerdeReplace, Clone, Debug, PartialEq, Eq)]
struct KeyBindings(Vec<KeyBinding>);

impl Default for KeyBindings {
    fn default() -> Self {
        Self(bindings::default_key_bindings())
    }
}

impl<'de> Deserialize<'de> for KeyBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(deserialize_bindings(deserializer, Self::default().0)?))
    }
}

#[derive(SerdeReplace, Clone, Debug, PartialEq, Eq)]
struct MouseBindings(Vec<MouseBinding>);

impl Default for MouseBindings {
    fn default() -> Self {
        Self(bindings::default_mouse_bindings())
    }
}

impl<'de> Deserialize<'de> for MouseBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(deserialize_bindings(deserializer, Self::default().0)?))
    }
}

fn deserialize_bindings<'a, D, T>(
    deserializer: D,
    mut default: Vec<Binding<T>>,
) -> Result<Vec<Binding<T>>, D::Error>
where
    D: Deserializer<'a>,
    T: Copy + Eq,
    Binding<T>: Deserialize<'a>,
{
    let values = Vec::<serde_yaml::Value>::deserialize(deserializer)?;

    // Skip all invalid values.
    let mut bindings = Vec::with_capacity(values.len());
    for value in values {
        match Binding::<T>::deserialize(value) {
            Ok(binding) => bindings.push(binding),
            Err(err) => {
                error!(target: LOG_TARGET_CONFIG, "Config error: {}; ignoring binding", err);
            },
        }
    }

    // Remove matching default bindings.
    for binding in bindings.iter() {
        default.retain(|b| !b.triggers_match(binding));
    }

    bindings.extend(default);

    Ok(bindings)
}

/// A delta for a point in a 2 dimensional plane.
#[derive(ConfigDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Delta<T: Default> {
    /// Horizontal change.
    pub x: T,
    /// Vertical change.
    pub y: T,
}
