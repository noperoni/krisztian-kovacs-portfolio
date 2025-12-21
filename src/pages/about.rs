use leptos::prelude::*;

use crate::i18n::use_i18n;

/// About page component - profile, skills, timeline, and contact CTA
#[component]
pub fn AboutPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="about-page">
            <PageHeader i18n=i18n />
            <ProfileSection i18n=i18n />
            <AboutSection i18n=i18n />
            <SkillsGrid i18n=i18n />
            <JourneyTimeline i18n=i18n />
            <ContactCTA i18n=i18n />
        </div>
    }
}

/// Page header with title and subtitle
#[component]
fn PageHeader(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <header class="page-header">
            <h1>{move || i18n.t().about_page_title}</h1>
            <p class="page-subtitle">{move || i18n.t().about_page_subtitle}</p>
        </header>
    }
}

/// Profile section with image and basic info
#[component]
fn ProfileSection(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="profile-section">
            <div class="profile-image">
                <img src="/images/krisztian-headshot.jpg" alt="Kovács Krisztián Géza" />
            </div>
            <div class="profile-info">
                <h2>"Kovács Krisztián Géza"</h2>
                <p class="profile-title">{move || i18n.t().about_profile_title}</p>
                <div class="profile-links">
                    <a
                        href="https://www.linkedin.com/in/kriszti%C3%A1n-g%C3%A9za-kov%C3%A1cs-2b72251a2/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="profile-link"
                    >
                        <span class="link-icon">"💼"</span>
                        <span>"LinkedIn"</span>
                    </a>
                    <a href="mailto:krisztian.kovacs.pro@pm.me" class="profile-link">
                        <span class="link-icon">"✉️"</span>
                        <span>"Email"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}

/// Introduction/bio section
#[component]
fn AboutSection(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="about-section">
            <h2>{move || i18n.t().about_intro_title}</h2>
            <p>{move || i18n.t().about_intro_p1}</p>
            <p>{move || i18n.t().about_intro_p2}</p>
            <p>{move || i18n.t().about_intro_p3}</p>
        </section>
    }
}

/// Skills and interests grid
#[component]
fn SkillsGrid(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="skills-section">
            <div class="skills-grid">
                <SkillCard
                    title=Signal::derive(move || i18n.t().about_skills_core_title.to_string())
                    skills=vec![
                        "Cloud Architecture (Azure, AWS, GCP)",
                        "Kubernetes & Container Orchestration",
                        "Infrastructure as Code (Terraform, Ansible)",
                        "IT Security & Compliance (GDPR, ISO 27001)",
                        "Enterprise System Integration",
                        "Project Management & Team Leadership",
                    ]
                />
                <SkillCard
                    title=Signal::derive(move || i18n.t().about_skills_interests_title.to_string())
                    skills=vec![
                        "Machine Learning & AI in Infrastructure",
                        "Green IT & Sustainable Technology",
                        "DevOps & GitOps Practices",
                        "Edge Computing & IoT",
                        "Blockchain & Distributed Systems",
                        "Open Source Contributions",
                    ]
                />
            </div>
        </section>
    }
}

/// Individual skill card component
#[component]
fn SkillCard(title: Signal<String>, skills: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="skill-card">
            <h3>{title}</h3>
            <ul class="skill-list">
                {skills.into_iter().map(|skill| view! {
                    <li>{skill}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}

/// Career journey timeline
#[component]
fn JourneyTimeline(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="journey-section">
            <h2 class="journey-title">{move || i18n.t().about_journey_title}</h2>
            <div class="timeline">
                <TimelineItem
                    date="2023 - Present"
                    title="Project Manager at Nomadia"
                    description=Signal::derive(move || i18n.t().about_timeline_nomadia.to_string())
                />
                <TimelineItem
                    date="2022 - 2023"
                    title="IT Administrator at Dynamips"
                    description=Signal::derive(move || i18n.t().about_timeline_dynamips.to_string())
                />
                <TimelineItem
                    date="2020 - 2022"
                    title="IT Technician at Entreprise CHARIER"
                    description=Signal::derive(move || i18n.t().about_timeline_charier.to_string())
                />
                <TimelineItemTranslated
                    date="2020 - 2025"
                    title=Signal::derive(move || i18n.t().about_timeline_education_title.to_string())
                    description=Signal::derive(move || i18n.t().about_timeline_education.to_string())
                />
                // Note: Education completed Sep 2025 - EPSI EISI diploma (Master's, RNCP Level 7)
            </div>
        </section>
    }
}

/// Timeline item with translated title
#[component]
fn TimelineItemTranslated(
    date: &'static str,
    title: Signal<String>,
    description: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="timeline-item">
            <div class="timeline-date">{date}</div>
            <div class="timeline-content">
                <h4>{title}</h4>
                <p>{description}</p>
            </div>
        </div>
    }
}

/// Individual timeline item
#[component]
fn TimelineItem(
    date: &'static str,
    #[prop(optional)] title: &'static str,
    description: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="timeline-item">
            <div class="timeline-date">{date}</div>
            <div class="timeline-content">
                <h4>{title}</h4>
                <p>{description}</p>
            </div>
        </div>
    }
}

/// Contact call-to-action section
#[component]
fn ContactCTA(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="contact-cta">
            <h2>{move || i18n.t().about_cta_title}</h2>
            <p>{move || i18n.t().about_cta_description}</p>
            <a href="mailto:krisztian.kovacs.pro@pm.me" class="btn btn-primary">
                {move || i18n.t().about_cta_button}
                <span>" →"</span>
            </a>
        </section>
    }
}
