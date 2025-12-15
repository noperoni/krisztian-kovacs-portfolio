use leptos::prelude::*;
use crate::themes::use_theme;

/// Theme toggle button component (cycles through Terminal -> Forge -> Bitart)
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_ctx = use_theme();

    let cycle_theme = move |_| {
        theme_ctx.theme.update(|t| *t = t.next());
    };

    let label = move || theme_ctx.theme.get().label();
    let icon = move || theme_ctx.theme.get().icon();

    view! {
        <button
            class="control-btn theme-toggle"
            id="themeToggle"
            on:click=cycle_theme
            title="Cycle theme"
        >
            <span class="toggle-icon">{icon}</span>
            <span class="toggle-label">{label}</span>
        </button>
    }
}
