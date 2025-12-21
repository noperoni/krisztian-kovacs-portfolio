use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::components::{ColorModeToggle, ContactFAB, LanguageToggle, ThemeToggle};
use crate::i18n::{provide_i18n_context, use_i18n};
use crate::pages::{AboutPage, BlogPage, BlogPostPage, BlogTagPage, CvPage, ProjectsPage};
use crate::themes::{provide_color_mode_context, provide_theme_context};

/// Sets the Content-Security-Policy header with the current nonce.
/// This component runs on the server and has no visual output.
#[component]
fn CspHeader() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use http::{header::HeaderName, HeaderValue};
        use leptos::nonce::use_nonce;
        use leptos_axum::ResponseOptions;

        if let Some(nonce) = use_nonce() {
            let nonce_str: &str = &nonce;
            let csp = format!(
                "default-src 'self'; \
                 script-src 'self' 'wasm-unsafe-eval' 'nonce-{}'; \
                 style-src 'self' 'unsafe-inline'; \
                 font-src 'self' data:; \
                 img-src 'self' data: https:; \
                 connect-src 'self'; \
                 frame-ancestors 'none'; \
                 base-uri 'self'; \
                 form-action 'self'",
                nonce_str
            );

            if let Some(response_options) = use_context::<ResponseOptions>() {
                response_options.insert_header(
                    HeaderName::from_static("content-security-policy"),
                    HeaderValue::from_str(&csp).unwrap_or_else(|_| {
                        HeaderValue::from_static("default-src 'self'")
                    }),
                );
            }
        }
    }

    // This component has no visual output
    ()
}

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
    // Provide theme and color mode contexts
    provide_theme_context();
    provide_color_mode_context();

    view! {
        // Set CSP header with nonce (server-side only, no visual output)
        <CspHeader/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        // sets the document title
        <Title text="Kovács Krisztián Géza - Portfolio"/>

        // content for this welcome page
        <Router>
            <Nav/>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("cv") view=CvPage/>
                    <Route path=StaticSegment("projects") view=ProjectsPage/>
                    <Route path=StaticSegment("blog") view=BlogPage/>
                    <Route path=(StaticSegment("blog"), StaticSegment("tags"), ParamSegment("tag")) view=BlogTagPage/>
                    <Route path=(StaticSegment("blog"), ParamSegment("slug")) view=BlogPostPage/>
                </Routes>
            </main>
        </Router>

        // Floating contact button (visible on all pages)
        <ContactFAB/>
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
                        <ThemeToggle/>
                        <ColorModeToggle/>
                        <LanguageToggle/>
                    </div>
                </div>
            </div>
        </nav>
    }
}

/// Renders the home page - Bold & Impactful design
#[component]
fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="hero">
            <div class="hero-container">
                <div class="hero-visual">
                    <div class="hero-image-wrapper">
                        <img src="/images/krisztian-headshot.jpg" alt="Kovács Krisztián Géza" class="hero-image" />
                        <div class="hero-image-glow"></div>
                    </div>
                </div>
                <div class="hero-content">
                    <div class="hero-badge">"Project Manager & Cloud Architect"</div>
                    <h1 class="hero-title">{move || i18n.t().hero_title}</h1>
                    <p class="hero-subtitle">{move || i18n.t().hero_subtitle}</p>
                    <p class="hero-description">{move || i18n.t().hero_description}</p>
                    <div class="cta-buttons">
                        <a href="/projects" class="btn btn-primary btn-lg">
                            <span class="btn-icon">"🚀"</span>
                            {move || i18n.t().hero_cta_projects}
                        </a>
                        <a href="/cv" class="btn btn-secondary btn-lg">
                            <span class="btn-icon">"📄"</span>
                            {move || i18n.t().hero_cta_cv}
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <section class="home-stats">
            <div class="stats-container">
                <div class="stat-card">
                    <span class="stat-number">"5+"</span>
                    <span class="stat-text">"Years Experience"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-number">"50+"</span>
                    <span class="stat-text">"Projects Delivered"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-number">"99.9%"</span>
                    <span class="stat-text">"Uptime SLA"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-number">"3"</span>
                    <span class="stat-text">"Cloud Platforms"</span>
                </div>
            </div>
        </section>

        <section class="features">
            <h2 class="features-title">"What I Do"</h2>
            <div class="features-grid">
                <FeatureCard
                    icon="☁️"
                    title=Signal::derive(move || i18n.t().feature_cloud_title.to_string())
                    description=Signal::derive(move || i18n.t().feature_cloud_desc.to_string())
                    highlight="Azure • AWS • GCP"
                />
                <FeatureCard
                    icon="🔒"
                    title=Signal::derive(move || i18n.t().feature_security_title.to_string())
                    description=Signal::derive(move || i18n.t().feature_security_desc.to_string())
                    highlight="GDPR • ISO 27001"
                />
                <FeatureCard
                    icon="🤖"
                    title=Signal::derive(move || i18n.t().feature_ai_title.to_string())
                    description=Signal::derive(move || i18n.t().feature_ai_desc.to_string())
                    highlight="MLOps • Automation"
                />
            </div>
        </section>

        <section class="home-cta">
            <div class="home-cta-content">
                <h2>"Ready to Build Something Great?"</h2>
                <p>"Let's discuss your next project and how I can help you achieve your goals."</p>
                <div class="home-cta-buttons">
                    <a href="mailto:kovacs@pilgrim.ovh" class="btn btn-primary btn-lg">
                        <span class="btn-icon">"✉️"</span>
                        "Get In Touch"
                    </a>
                    <a href="/about" class="btn btn-outline btn-lg">
                        "Learn More About Me"
                    </a>
                </div>
            </div>
        </section>
    }
}

/// Feature card component - Enhanced
#[component]
fn FeatureCard(
    icon: &'static str,
    title: Signal<String>,
    description: Signal<String>,
    highlight: &'static str,
) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3>{title}</h3>
            <p class="feature-description">{description}</p>
            <span class="feature-highlight">{highlight}</span>
        </div>
    }
}
