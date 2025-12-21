use leptos::prelude::*;

use crate::i18n::use_i18n;

/// About page component - Bold & Impactful design
#[component]
pub fn AboutPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="about-page">
            <HeroProfile i18n=i18n />
            <StatsBar />
            <AboutSection i18n=i18n />
            <SkillsShowcase i18n=i18n />
            <JourneyTimeline i18n=i18n />
            <ContactCTA i18n=i18n />
        </div>
    }
}

/// Hero profile - Bold first impression
#[component]
fn HeroProfile(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="hero-profile">
            <div class="hero-profile-content">
                <div class="profile-image-wrapper">
                    <img src="/images/krisztian-headshot.jpg" alt="Kovács Krisztián Géza" class="profile-image" />
                    <div class="profile-image-ring"></div>
                </div>
                <div class="profile-details">
                    <h1 class="profile-name">"Kovács Krisztián Géza"</h1>
                    <p class="profile-title">{move || i18n.t().about_profile_title}</p>
                    <p class="profile-tagline">{move || i18n.t().hero_description}</p>
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
                        <a href="mailto:kovacs@pilgrim.ovh" class="profile-link">
                            <span class="link-icon">"✉️"</span>
                            <span>"Email"</span>
                        </a>
                        <a href="/cv" class="profile-link">
                            <span class="link-icon">"📄"</span>
                            <span>"CV"</span>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Stats bar - Key metrics that impress
#[component]
fn StatsBar() -> impl IntoView {
    view! {
        <section class="stats-bar">
            <div class="stat-item">
                <span class="stat-value">"5+"</span>
                <span class="stat-label">"Years Experience"</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">"50+"</span>
                <span class="stat-label">"Projects Delivered"</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">"99.9%"</span>
                <span class="stat-label">"Uptime Achieved"</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">"3"</span>
                <span class="stat-label">"Cloud Platforms"</span>
            </div>
        </section>
    }
}

/// Introduction/bio section with impact
#[component]
fn AboutSection(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="about-section">
            <h2 class="section-heading">{move || i18n.t().about_intro_title}</h2>
            <div class="about-content">
                <p class="about-lead">{move || i18n.t().about_intro_p1}</p>
                <p>{move || i18n.t().about_intro_p2}</p>
                <p>{move || i18n.t().about_intro_p3}</p>
            </div>
        </section>
    }
}

/// Skills showcase with visual tags
#[component]
fn SkillsShowcase(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="skills-showcase">
            <div class="skills-category">
                <h3 class="skills-category-title">
                    <span class="category-icon">"⚡"</span>
                    {move || i18n.t().about_skills_core_title}
                </h3>
                <div class="skills-tags">
                    <SkillTag icon="☁️" name="Cloud Architecture" detail="Azure, AWS, GCP" />
                    <SkillTag icon="🐳" name="Kubernetes" detail="Container Orchestration" />
                    <SkillTag icon="🔧" name="IaC" detail="Terraform, Ansible" />
                    <SkillTag icon="🔒" name="Security" detail="GDPR, ISO 27001" />
                    <SkillTag icon="🔗" name="Integration" detail="Enterprise Systems" />
                    <SkillTag icon="👥" name="Leadership" detail="Project & Team" />
                </div>
            </div>
            <div class="skills-category">
                <h3 class="skills-category-title">
                    <span class="category-icon">"🚀"</span>
                    {move || i18n.t().about_skills_interests_title}
                </h3>
                <div class="skills-tags">
                    <SkillTag icon="🤖" name="AI/ML" detail="Infrastructure AI" />
                    <SkillTag icon="🌱" name="Green IT" detail="Sustainable Tech" />
                    <SkillTag icon="🔄" name="DevOps" detail="GitOps Practices" />
                    <SkillTag icon="📡" name="Edge" detail="IoT Computing" />
                    <SkillTag icon="⛓️" name="Blockchain" detail="Distributed Systems" />
                    <SkillTag icon="💻" name="Open Source" detail="Contributions" />
                </div>
            </div>
        </section>
    }
}

/// Individual skill tag with icon
#[component]
fn SkillTag(icon: &'static str, name: &'static str, detail: &'static str) -> impl IntoView {
    view! {
        <div class="skill-tag">
            <span class="skill-icon">{icon}</span>
            <div class="skill-info">
                <span class="skill-name">{name}</span>
                <span class="skill-detail">{detail}</span>
            </div>
        </div>
    }
}

/// Career journey timeline - Enhanced visual
#[component]
fn JourneyTimeline(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="journey-section">
            <h2 class="section-heading">{move || i18n.t().about_journey_title}</h2>
            <div class="timeline">
                <TimelineItem
                    icon="🎯"
                    date="2023 - Present"
                    title="Project Manager"
                    company="Nomadia"
                    description=Signal::derive(move || i18n.t().about_timeline_nomadia.to_string())
                    is_current=true
                />
                <TimelineItem
                    icon="🖥️"
                    date="2022 - 2023"
                    title="IT Administrator"
                    company="Dynamips"
                    description=Signal::derive(move || i18n.t().about_timeline_dynamips.to_string())
                    is_current=false
                />
                <TimelineItem
                    icon="🔧"
                    date="2020 - 2022"
                    title="IT Technician"
                    company="Entreprise CHARIER"
                    description=Signal::derive(move || i18n.t().about_timeline_charier.to_string())
                    is_current=false
                />
                <TimelineItemEducation
                    icon="🎓"
                    date="2020 - 2025"
                    title=Signal::derive(move || i18n.t().about_timeline_education_title.to_string())
                    description=Signal::derive(move || i18n.t().about_timeline_education.to_string())
                />
            </div>
        </section>
    }
}

/// Timeline item for work experience
#[component]
fn TimelineItem(
    icon: &'static str,
    date: &'static str,
    title: &'static str,
    company: &'static str,
    description: Signal<String>,
    is_current: bool,
) -> impl IntoView {
    view! {
        <div class={if is_current { "timeline-item timeline-item-current" } else { "timeline-item" }}>
            <div class="timeline-marker">
                <span class="timeline-icon">{icon}</span>
            </div>
            <div class="timeline-content">
                <div class="timeline-header">
                    <span class="timeline-date">{date}</span>
                    {is_current.then(|| view! { <span class="timeline-badge">"Current"</span> })}
                </div>
                <h4 class="timeline-title">{title}</h4>
                <p class="timeline-company">{company}</p>
                <p class="timeline-description">{description}</p>
            </div>
        </div>
    }
}

/// Timeline item for education
#[component]
fn TimelineItemEducation(
    icon: &'static str,
    date: &'static str,
    title: Signal<String>,
    description: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="timeline-item timeline-item-education">
            <div class="timeline-marker">
                <span class="timeline-icon">{icon}</span>
            </div>
            <div class="timeline-content">
                <div class="timeline-header">
                    <span class="timeline-date">{date}</span>
                    <span class="timeline-badge timeline-badge-complete">"Completed"</span>
                </div>
                <h4 class="timeline-title">{title}</h4>
                <p class="timeline-description">{description}</p>
            </div>
        </div>
    }
}

/// Contact call-to-action - Bold and impactful
#[component]
fn ContactCTA(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="contact-cta">
            <div class="cta-content">
                <h2 class="cta-title">{move || i18n.t().about_cta_title}</h2>
                <p class="cta-description">{move || i18n.t().about_cta_description}</p>
                <div class="cta-buttons">
                    <a href="mailto:kovacs@pilgrim.ovh" class="btn btn-primary btn-lg">
                        <span class="btn-icon">"✉️"</span>
                        {move || i18n.t().about_cta_button}
                    </a>
                    <a href="https://www.linkedin.com/in/kriszti%C3%A1n-g%C3%A9za-kov%C3%A1cs-2b72251a2/"
                       target="_blank"
                       rel="noopener noreferrer"
                       class="btn btn-secondary btn-lg">
                        <span class="btn-icon">"💼"</span>
                        "LinkedIn"
                    </a>
                </div>
            </div>
        </section>
    }
}
