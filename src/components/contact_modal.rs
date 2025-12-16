//! Contact form modal component
//!
//! Floating action button (FAB) that opens a modal contact form.

use leptos::prelude::*;

use crate::i18n::use_i18n;
use crate::server_fns::{submit_contact, ContactFormInput};

/// Floating action button that opens the contact modal
#[component]
pub fn ContactFAB() -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        // Floating Action Button
        <button
            class="contact-fab"
            on:click=move |_| is_open.set(true)
            aria-label="Open contact form"
        >
            <span class="fab-icon">"@"</span>
        </button>

        // Modal (rendered when open)
        <Show when=move || is_open.get()>
            <ContactModal on_close=move || is_open.set(false) />
        </Show>
    }
}

/// Contact form modal overlay
#[component]
fn ContactModal<F>(on_close: F) -> impl IntoView
where
    F: Fn() + Clone + 'static,
{
    let i18n = use_i18n();

    // Form state
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let subject = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let honeypot = RwSignal::new(String::new()); // Hidden field for bots

    // Submission state
    let is_submitting = RwSignal::new(false);
    let result_message = RwSignal::new(Option::<(bool, String)>::None);

    // Create server action
    let submit_action = Action::new(move |input: &ContactFormInput| {
        let input = input.clone();
        async move { submit_contact(input).await }
    });

    // Handle action result
    let i18n_for_effect = i18n;
    Effect::new(move || {
        if let Some(result) = submit_action.value().get() {
            is_submitting.set(false);
            match result {
                Ok(res) => {
                    let msg = get_message_for_key(&res.message_key, i18n_for_effect);
                    result_message.set(Some((res.success, msg)));
                    if res.success {
                        // Clear form on success
                        name.set(String::new());
                        email.set(String::new());
                        subject.set(String::new());
                        message.set(String::new());
                    }
                }
                Err(_) => {
                    result_message.set(Some((
                        false,
                        i18n_for_effect.t().contact_error_generic.to_string(),
                    )));
                }
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        is_submitting.set(true);
        result_message.set(None);

        submit_action.dispatch(ContactFormInput {
            name: name.get(),
            email: email.get(),
            subject: subject.get(),
            message: message.get(),
            website: honeypot.get(), // Honeypot
        });
    };

    // Close on escape key
    let on_close_escape = on_close.clone();
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close_escape();
        }
    };

    // Close on backdrop click (the backdrop closes, modal stops propagation)
    let on_close_backdrop = on_close.clone();

    view! {
        <div
            class="contact-modal-backdrop"
            on:click=move |_| on_close_backdrop()
            on:keydown=on_keydown
            tabindex="-1"
        >
            <div class="contact-modal" role="dialog" aria-modal="true" on:click=|ev| ev.stop_propagation()>
                <div class="modal-header">
                    <h2>{move || i18n.t().contact_title}</h2>
                    <button
                        class="modal-close"
                        on:click=move |_| on_close()
                        aria-label="Close"
                    >
                        "x"
                    </button>
                </div>

                <form class="contact-form" on:submit=on_submit>
                    // Honeypot field (hidden from users, visible to bots)
                    <div class="contact-honeypot" aria-hidden="true">
                        <input
                            type="text"
                            name="website"
                            tabindex="-1"
                            autocomplete="off"
                            bind:value=honeypot
                        />
                    </div>

                    <div class="form-group">
                        <label for="contact-name">{move || i18n.t().contact_name}</label>
                        <input
                            type="text"
                            id="contact-name"
                            name="name"
                            required
                            maxlength="255"
                            bind:value=name
                            disabled=move || is_submitting.get()
                        />
                    </div>

                    <div class="form-group">
                        <label for="contact-email">{move || i18n.t().contact_email}</label>
                        <input
                            type="email"
                            id="contact-email"
                            name="email"
                            required
                            maxlength="255"
                            bind:value=email
                            disabled=move || is_submitting.get()
                        />
                    </div>

                    <div class="form-group">
                        <label for="contact-subject">{move || i18n.t().contact_subject}</label>
                        <input
                            type="text"
                            id="contact-subject"
                            name="subject"
                            maxlength="500"
                            bind:value=subject
                            disabled=move || is_submitting.get()
                        />
                    </div>

                    <div class="form-group">
                        <label for="contact-message">{move || i18n.t().contact_message}</label>
                        <textarea
                            id="contact-message"
                            name="message"
                            required
                            maxlength="5000"
                            rows="5"
                            bind:value=message
                            disabled=move || is_submitting.get()
                        />
                    </div>

                    // Result message
                    <Show when=move || result_message.get().is_some()>
                        {move || {
                            let (success, msg) = result_message.get().unwrap();
                            let class = if success { "form-message success" } else { "form-message error" };
                            view! {
                                <div class=class>
                                    {msg}
                                </div>
                            }
                        }}
                    </Show>

                    <button
                        type="submit"
                        class="btn btn-primary submit-btn"
                        disabled=move || is_submitting.get()
                    >
                        {move || {
                            if is_submitting.get() {
                                i18n.t().contact_sending.to_string()
                            } else {
                                i18n.t().contact_send.to_string()
                            }
                        }}
                    </button>
                </form>
            </div>
        </div>
    }
}

/// Helper to get translated message for a key
fn get_message_for_key(key: &str, i18n: crate::i18n::I18nContext) -> String {
    match key {
        "contact_success" => i18n.t().contact_success.to_string(),
        "contact_error_name" => i18n.t().contact_error_name.to_string(),
        "contact_error_email" => i18n.t().contact_error_email.to_string(),
        "contact_error_message" => i18n.t().contact_error_message.to_string(),
        "contact_error_subject" => i18n.t().contact_error_subject.to_string(),
        "contact_error_rate_limit" => i18n.t().contact_error_rate_limit.to_string(),
        _ => i18n.t().contact_error_generic.to_string(),
    }
}
