use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::LanguageToggle;
use crate::i18n::{provide_i18n_context, use_i18n};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Provide i18n context for translations
    provide_i18n_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        // sets the document title
        <Title text="Krisztián Kovács - Portfolio"/>

        // content for this welcome page
        <Router>
            <Nav/>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Navigation bar component
#[component]
fn Nav() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <nav>
            <div class="nav-container">
                <div class="nav-content">
                    <ul class="nav-links">
                        <li><a href="/" class="active">{move || i18n.t().nav_home}</a></li>
                        <li><a href="/about">{move || i18n.t().nav_about}</a></li>
                        <li><a href="/cv">{move || i18n.t().nav_cv}</a></li>
                        <li><a href="/projects">{move || i18n.t().nav_projects}</a></li>
                        <li><a href="/blog">{move || i18n.t().nav_blog}</a></li>
                    </ul>
                    <div class="control-panel">
                        <LanguageToggle/>
                    </div>
                </div>
            </div>
        </nav>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="hero">
            <div class="hero-content">
                <h1>{move || i18n.t().hero_title}</h1>
                <p class="subtitle">{move || i18n.t().hero_subtitle}</p>
                <p class="hero-description">{move || i18n.t().hero_description}</p>
                <div class="cta-buttons">
                    <a href="/projects" class="btn btn-primary">
                        {move || i18n.t().hero_cta_projects}
                        <span>" →"</span>
                    </a>
                    <a href="/cv" class="btn btn-secondary">
                        {move || i18n.t().hero_cta_cv}
                        <span>" ↓"</span>
                    </a>
                </div>
            </div>
        </section>

        <section class="features">
            <div class="features-grid">
                <FeatureCard
                    icon="🚀"
                    title=Signal::derive(move || i18n.t().feature_cloud_title.to_string())
                    description=Signal::derive(move || i18n.t().feature_cloud_desc.to_string())
                />
                <FeatureCard
                    icon="🔒"
                    title=Signal::derive(move || i18n.t().feature_security_title.to_string())
                    description=Signal::derive(move || i18n.t().feature_security_desc.to_string())
                />
                <FeatureCard
                    icon="🤖"
                    title=Signal::derive(move || i18n.t().feature_ai_title.to_string())
                    description=Signal::derive(move || i18n.t().feature_ai_desc.to_string())
                />
            </div>
        </section>
    }
}

/// Feature card component
#[component]
fn FeatureCard(
    icon: &'static str,
    title: Signal<String>,
    description: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3>{title}</h3>
            <p>{description}</p>
        </div>
    }
}
