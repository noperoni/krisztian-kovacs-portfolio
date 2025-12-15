use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Color mode (light/dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ColorMode {
    #[default]
    Dark,
    Light,
}

impl ColorMode {
    /// Get the color mode code string (used for data attribute)
    pub fn code(&self) -> &'static str {
        match self {
            ColorMode::Dark => "dark",
            ColorMode::Light => "light",
        }
    }

    /// Parse from string, defaults to Dark
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "light" => ColorMode::Light,
            _ => ColorMode::Dark,
        }
    }

    /// Toggle between light and dark
    pub fn toggle(&self) -> Self {
        match self {
            ColorMode::Dark => ColorMode::Light,
            ColorMode::Light => ColorMode::Dark,
        }
    }

    /// Display label for the toggle button
    pub fn label(&self) -> &'static str {
        match self {
            ColorMode::Dark => "DARK",
            ColorMode::Light => "LIGHT",
        }
    }

    /// Icon for the color mode
    pub fn icon(&self) -> &'static str {
        match self {
            ColorMode::Dark => "🌙",
            ColorMode::Light => "☀️",
        }
    }
}

/// Color mode context that holds the current color mode
#[derive(Clone, Copy)]
pub struct ColorModeContext {
    pub mode: RwSignal<ColorMode>,
}

/// Provide color mode context to the application
pub fn provide_color_mode_context() {
    let initial_mode = load_color_mode_from_storage();
    let mode = RwSignal::new(initial_mode);

    // Apply initial color mode to document
    apply_color_mode_to_document(initial_mode);

    // Persist color mode changes to localStorage and update document
    Effect::new(move || {
        let m = mode.get();
        save_color_mode_to_storage(m);
        apply_color_mode_to_document(m);
    });

    provide_context(ColorModeContext { mode });
}

/// Hook to access color mode context
pub fn use_color_mode() -> ColorModeContext {
    expect_context::<ColorModeContext>()
}

/// Load color mode from localStorage, with system preference fallback (client-side only)
fn load_color_mode_from_storage() -> ColorMode {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            // First check localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(mode)) = storage.get_item("color-mode") {
                    return ColorMode::from_code(&mode);
                }
            }
            // Fallback to system preference
            if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
                if media_query.matches() {
                    return ColorMode::Dark;
                } else {
                    return ColorMode::Light;
                }
            }
        }
    }
    ColorMode::default()
}

/// Save color mode to localStorage (client-side only)
fn save_color_mode_to_storage(mode: ColorMode) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("color-mode", mode.code());
            }
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = mode; // Suppress unused warning on server
    }
}

/// Apply color mode to document element (client-side only)
fn apply_color_mode_to_document(mode: ColorMode) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("data-color", mode.code());
                }
            }
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = mode; // Suppress unused warning on server
    }
}
