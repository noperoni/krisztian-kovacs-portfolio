mod color_mode;

pub use color_mode::{provide_color_mode_context, use_color_mode, ColorMode, ColorModeContext};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Available themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Terminal,  // Terminal Artisan - Warm CRT vibes, JetBrains Mono
    Forge,     // Forge & Steel - Industrial, bold Bebas Neue typography
    Bitart,    // Bitart RPG - Press Start 2P font, RPG HUD style
}

impl Theme {
    /// Get the theme code string (used for data attribute)
    pub fn code(&self) -> &'static str {
        match self {
            Theme::Terminal => "terminal",
            Theme::Forge => "forge",
            Theme::Bitart => "bitart",
        }
    }

    /// Parse from string, defaults to Terminal
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "forge" => Theme::Forge,
            "bitart" => Theme::Bitart,
            _ => Theme::Terminal,
        }
    }

    /// Cycle to the next theme
    pub fn next(&self) -> Self {
        match self {
            Theme::Terminal => Theme::Forge,
            Theme::Forge => Theme::Bitart,
            Theme::Bitart => Theme::Terminal,
        }
    }

    /// Display label for the toggle button
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Terminal => "TERM",
            Theme::Forge => "PRO",
            Theme::Bitart => "FUN",
        }
    }

    /// Icon for the theme
    pub fn icon(&self) -> &'static str {
        match self {
            Theme::Terminal => ">_",
            Theme::Forge => "⚒",
            Theme::Bitart => "🎮",
        }
    }
}

/// Theme context that holds the current theme
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
}

/// Provide theme context to the application
pub fn provide_theme_context() {
    let initial_theme = load_theme_from_storage();
    let theme = RwSignal::new(initial_theme);

    // Apply initial theme to document
    apply_theme_to_document(initial_theme);

    // Persist theme changes to localStorage and update document
    Effect::new(move || {
        let t = theme.get();
        save_theme_to_storage(t);
        apply_theme_to_document(t);
    });

    provide_context(ThemeContext { theme });
}

/// Hook to access theme context
pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

/// Load theme from localStorage (client-side only)
fn load_theme_from_storage() -> Theme {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme)) = storage.get_item("theme") {
                    return Theme::from_code(&theme);
                }
            }
        }
    }
    Theme::default()
}

/// Save theme to localStorage (client-side only)
fn save_theme_to_storage(theme: Theme) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", theme.code());
            }
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = theme; // Suppress unused warning on server
    }
}

/// Apply theme to document element (client-side only)
fn apply_theme_to_document(theme: Theme) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("data-theme", theme.code());
                }
            }
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = theme; // Suppress unused warning on server
    }
}
