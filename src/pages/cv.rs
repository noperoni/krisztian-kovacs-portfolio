use leptos::prelude::*;

use crate::i18n::use_i18n;

/// CV page component - full curriculum vitae with print support
#[component]
pub fn CvPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="cv-page">
            <CvHero i18n=i18n />
            <ProfessionalSummary i18n=i18n />
            <TechnicalExpertise i18n=i18n />
            <ProfessionalExperience i18n=i18n />
            <KeyProjects i18n=i18n />
            <EducationCertifications i18n=i18n />
            <Languages i18n=i18n />
        </div>
    }
}

/// CV Hero with name, title, and contact info
#[component]
fn CvHero(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <header class="cv-hero">
            <h1>"Kovács Krisztián Géza"</h1>
            <p class="cv-subtitle">{move || i18n.t().cv_subtitle}</p>
            <div class="cv-contact">
                <a href="mailto:krisztian.kovacs.pro@pm.me" class="contact-item">
                    <span class="contact-icon">"📧"</span>
                    <span>"krisztian.kovacs.pro@pm.me"</span>
                </a>
                <span class="contact-item">
                    <span class="contact-icon">"📍"</span>
                    <span>"France"</span>
                </span>
                <a
                    href="https://www.linkedin.com/in/kriszti%C3%A1n-g%C3%A9za-kov%C3%A1cs-2b72251a2/"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="contact-item"
                >
                    <span class="contact-icon">"💼"</span>
                    <span>"LinkedIn"</span>
                </a>
            </div>
        </header>
    }
}

/// Professional Summary section
#[component]
fn ProfessionalSummary(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="cv-section">
            <h2 class="section-title">{move || i18n.t().cv_summary_title}</h2>
            <p class="summary-text">{move || i18n.t().cv_summary_text}</p>
        </section>
    }
}

/// Technical Expertise grid
#[component]
fn TechnicalExpertise(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="cv-section">
            <h2 class="section-title">{move || i18n.t().cv_expertise_title}</h2>
            <div class="expertise-grid">
                <ExpertiseCard
                    title=Signal::derive(move || i18n.t().cv_expertise_infra_title.to_string())
                    skills=vec![
                        "Hyper-V, ESXi, Proxmox",
                        "Kubernetes, Docker",
                        "Microsoft Azure, OVH, GCP",
                    ]
                />
                <ExpertiseCardI18n
                    title=Signal::derive(move || i18n.t().cv_expertise_support_title.to_string())
                    skills=vec![
                        Signal::derive(move || i18n.t().cv_skill_support_levels.to_string()),
                        Signal::derive(move || i18n.t().cv_skill_audit.to_string()),
                        Signal::derive(move || "ITIL Framework".to_string()),
                        Signal::derive(move || i18n.t().cv_skill_project_mgmt.to_string()),
                    ]
                />
                <ExpertiseCard
                    title=Signal::derive(move || i18n.t().cv_expertise_tools_title.to_string())
                    skills=vec![
                        "Microsoft 365, Azure AD",
                        "VEEAM Backup & Replication",
                        "Sophos XGS, BitLocker, MFA",
                        "PowerShell, Terraform, Ansible",
                        "Prometheus, Grafana, Zabbix",
                    ]
                />
            </div>
        </section>
    }
}

/// Expertise category card with static skills
#[component]
fn ExpertiseCard(title: Signal<String>, skills: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="expertise-card">
            <h4>{title}</h4>
            <ul class="expertise-list">
                {skills.into_iter().map(|skill| view! {
                    <li>{skill}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}

/// Expertise category card with translated skills
#[component]
fn ExpertiseCardI18n(title: Signal<String>, skills: Vec<Signal<String>>) -> impl IntoView {
    view! {
        <div class="expertise-card">
            <h4>{title}</h4>
            <ul class="expertise-list">
                {skills.into_iter().map(|skill| view! {
                    <li>{skill}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}

/// Professional Experience timeline
#[component]
fn ProfessionalExperience(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="cv-section">
            <h2 class="section-title">{move || i18n.t().cv_experience_title}</h2>
            <div class="cv-timeline">
                // Current role - Project Manager
                <ExperienceItem
                    title=Signal::derive(move || i18n.t().cv_exp_nomadia_pm_title.to_string())
                    company="Nomadia"
                    date=Signal::derive(move || i18n.t().cv_exp_nomadia_pm_date.to_string())
                    location="Nantes, France"
                    responsibilities=vec![
                        Signal::derive(move || i18n.t().cv_exp_nomadia_pm_r1.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_nomadia_pm_r2.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_nomadia_pm_r3.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_nomadia_pm_r4.to_string()),
                    ]
                />
                // Previous role - IT Expert (Work-Study)
                <ExperienceItem
                    title=Signal::derive(move || i18n.t().cv_exp_nomadia_title.to_string())
                    company="Nomadia"
                    date=Signal::derive(move || i18n.t().cv_exp_nomadia_date.to_string())
                    location="Nantes, France"
                    responsibilities=vec![
                        Signal::derive(move || i18n.t().cv_exp_nomadia_r1.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_nomadia_r2.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_nomadia_r3.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_nomadia_r4.to_string()),
                    ]
                />
                <ExperienceItem
                    title=Signal::derive(move || i18n.t().cv_exp_dynamips_title.to_string())
                    company="Dynamips"
                    date=Signal::derive(move || i18n.t().cv_exp_dynamips_date.to_string())
                    location="Saint-Herblain, France"
                    responsibilities=vec![
                        Signal::derive(move || i18n.t().cv_exp_dynamips_r1.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_dynamips_r2.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_dynamips_r3.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_dynamips_r4.to_string()),
                    ]
                />
                <ExperienceItem
                    title=Signal::derive(move || i18n.t().cv_exp_charier_title.to_string())
                    company="Entreprise CHARIER"
                    date=Signal::derive(move || i18n.t().cv_exp_charier_date.to_string())
                    location="Couëron, France"
                    responsibilities=vec![
                        Signal::derive(move || i18n.t().cv_exp_charier_r1.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_charier_r2.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_charier_r3.to_string()),
                        Signal::derive(move || i18n.t().cv_exp_charier_r4.to_string()),
                    ]
                />
            </div>
        </section>
    }
}

/// Single experience timeline item
#[component]
fn ExperienceItem(
    title: Signal<String>,
    company: &'static str,
    date: Signal<String>,
    location: &'static str,
    responsibilities: Vec<Signal<String>>,
) -> impl IntoView {
    view! {
        <div class="experience-item">
            <h3 class="job-title">{title}</h3>
            <div class="company">{company}</div>
            <div class="date-location">
                {date}
                <span>" | "</span>
                {location}
            </div>
            <ul class="responsibilities">
                {responsibilities.into_iter().map(|r| view! {
                    <li>{r}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}

/// Key Projects grid
#[component]
fn KeyProjects(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="cv-section">
            <h2 class="section-title">{move || i18n.t().cv_projects_title}</h2>
            <div class="projects-grid">
                <ProjectCard
                    title=Signal::derive(move || i18n.t().cv_proj_k8s_title.to_string())
                    date="Apr 2025 - Jul 2025"
                    description=Signal::derive(move || i18n.t().cv_proj_k8s_desc.to_string())
                    tech="Kubernetes, Terraform, Ansible, Prometheus"
                />
                <ProjectCard
                    title=Signal::derive(move || i18n.t().cv_proj_modernization_title.to_string())
                    date="Oct 2024 - Mar 2025"
                    description=Signal::derive(move || i18n.t().cv_proj_modernization_desc.to_string())
                    tech="ITIL, ISO 27001, Lean IT, ERP"
                />
                <ProjectCard
                    title=Signal::derive(move || i18n.t().cv_proj_endpoint_title.to_string())
                    date="Mar 2024 - Oct 2024"
                    description=Signal::derive(move || i18n.t().cv_proj_endpoint_desc.to_string())
                    tech="Intune, Azure AD, BitLocker, PowerShell"
                />
                <ProjectCard
                    title=Signal::derive(move || i18n.t().cv_proj_multicloud_title.to_string())
                    date="Oct 2024"
                    description=Signal::derive(move || i18n.t().cv_proj_multicloud_desc.to_string())
                    tech="Azure, Infrastructure as Code, DevOps"
                />
            </div>
        </section>
    }
}

/// Project card component
#[component]
fn ProjectCard(
    title: Signal<String>,
    date: &'static str,
    description: Signal<String>,
    tech: &'static str,
) -> impl IntoView {
    view! {
        <div class="project-card">
            <h3>{title}</h3>
            <div class="project-date">{date}</div>
            <p class="project-description">{description}</p>
            <div class="project-tech">{tech}</div>
        </div>
    }
}

/// Education and Certifications section
#[component]
fn EducationCertifications(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="cv-section">
            <h2 class="section-title">{move || i18n.t().cv_education_title}</h2>
            <div class="education-grid">
                <div class="education-column">
                    <h3>{move || i18n.t().cv_education_heading}</h3>
                    <div class="education-item">
                        <div class="degree">{move || i18n.t().cv_edu_master_title}</div>
                        <div class="school">"EPSI Nantes | 2023 - 2025"</div>
                        <div class="level">{move || i18n.t().cv_edu_master_level}</div>
                    </div>
                    <div class="education-item">
                        <div class="degree">{move || i18n.t().cv_edu_bachelor_title}</div>
                        <div class="school">"EPSI Nantes | 2022 - 2023"</div>
                        <div class="level">{move || i18n.t().cv_edu_bachelor_level}</div>
                    </div>
                </div>
                <div class="certifications-column">
                    <h3>{move || i18n.t().cv_certifications_heading}</h3>
                    <ul class="certifications-list">
                        <li>"Sophos Engineer ET80"</li>
                        <li>"Microsoft MS-900"</li>
                        <li>"Microsoft AZ-900"</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}

/// Languages section
#[component]
fn Languages(i18n: crate::i18n::I18nContext) -> impl IntoView {
    view! {
        <section class="cv-section">
            <h2 class="section-title">{move || i18n.t().cv_languages_title}</h2>
            <div class="languages-list">
                <LanguageItem
                    name=Signal::derive(move || i18n.t().cv_lang_english.to_string())
                    level=Signal::derive(move || i18n.t().cv_lang_native.to_string())
                />
                <LanguageItem
                    name=Signal::derive(move || i18n.t().cv_lang_french.to_string())
                    level=Signal::derive(move || i18n.t().cv_lang_professional.to_string())
                />
                <LanguageItem
                    name=Signal::derive(move || i18n.t().cv_lang_hungarian.to_string())
                    level=Signal::derive(move || i18n.t().cv_lang_native.to_string())
                />
                <LanguageItem
                    name=Signal::derive(move || i18n.t().cv_lang_romanian.to_string())
                    level=Signal::derive(move || i18n.t().cv_lang_native.to_string())
                />
                <LanguageItem
                    name=Signal::derive(move || i18n.t().cv_lang_german.to_string())
                    level=Signal::derive(move || i18n.t().cv_lang_limited.to_string())
                />
            </div>
        </section>
    }
}

/// Language item component
#[component]
fn LanguageItem(name: Signal<String>, level: Signal<String>) -> impl IntoView {
    view! {
        <div class="language-item">
            <span class="language-name">{name}</span>
            <span class="language-level">{level}</span>
        </div>
    }
}
