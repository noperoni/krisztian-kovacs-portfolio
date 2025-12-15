mod translations;

pub use translations::Translations;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    En,
    Fr,
}

impl Language {
    /// Get the language code string
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Fr => "fr",
        }
    }

    /// Parse from string, defaults to English
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "fr" => Language::Fr,
            _ => Language::En,
        }
    }

    /// Toggle to the other language
    pub fn toggle(&self) -> Self {
        match self {
            Language::En => Language::Fr,
            Language::Fr => Language::En,
        }
    }

    /// Display label for the toggle button
    pub fn label(&self) -> &'static str {
        match self {
            Language::En => "EN",
            Language::Fr => "FR",
        }
    }
}

/// i18n context that holds the current language and translations
#[derive(Clone, Copy)]
pub struct I18nContext {
    pub language: RwSignal<Language>,
}

impl I18nContext {
    /// Get translations for the current language
    pub fn t(&self) -> Translations {
        Translations::for_language(self.language.get())
    }
}

/// Provide i18n context to the application
pub fn provide_i18n_context() {
    let initial_lang = load_language_from_storage();
    let language = RwSignal::new(initial_lang);

    // Persist language changes to localStorage
    Effect::new(move || {
        let lang = language.get();
        save_language_to_storage(lang);
    });

    provide_context(I18nContext { language });
}

/// Hook to access i18n context
pub fn use_i18n() -> I18nContext {
    expect_context::<I18nContext>()
}

/// Load language from localStorage (client-side only)
fn load_language_from_storage() -> Language {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(lang)) = storage.get_item("language") {
                    return Language::from_code(&lang);
                }
            }
        }
    }
    Language::default()
}

/// Save language to localStorage (client-side only)
fn save_language_to_storage(lang: Language) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("language", lang.code());
            }
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = lang; // Suppress unused warning on server
    }
}
