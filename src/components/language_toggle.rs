use leptos::prelude::*;
use crate::i18n::use_i18n;

/// Language toggle button component
#[component]
pub fn LanguageToggle() -> impl IntoView {
    let i18n = use_i18n();

    let toggle_language = move |_| {
        i18n.language.update(|lang| *lang = lang.toggle());
    };

    let label = move || i18n.language.get().label();

    view! {
        <button
            class="control-btn"
            id="langToggle"
            on:click=toggle_language
        >
            {label}
        </button>
    }
}
