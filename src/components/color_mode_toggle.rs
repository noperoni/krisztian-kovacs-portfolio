use leptos::prelude::*;
use crate::themes::use_color_mode;

/// Color mode toggle button component (toggles between light and dark)
#[component]
pub fn ColorModeToggle() -> impl IntoView {
    let color_ctx = use_color_mode();

    let toggle_mode = move |_| {
        color_ctx.mode.update(|m| *m = m.toggle());
    };

    let icon = move || color_ctx.mode.get().icon();

    view! {
        <button
            class="control-btn color-mode-toggle"
            id="colorModeToggle"
            on:click=toggle_mode
            title="Toggle light/dark mode"
        >
            <span class="toggle-icon">{icon}</span>
        </button>
    }
}
