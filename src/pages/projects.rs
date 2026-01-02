use leptos::prelude::*;

use crate::i18n::{use_i18n, I18nContext};
use crate::server_fns::{get_github_repos, GithubRepoDisplay};

// ============================================================================
// DATA MODEL
// ============================================================================

/// Project category for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectCategory {
    Cloud,
    Security,
    Automation,
}

impl ProjectCategory {
    pub fn all() -> &'static [ProjectCategory] {
        &[
            ProjectCategory::Cloud,
            ProjectCategory::Security,
            ProjectCategory::Automation,
        ]
    }

}

/// A single project stat (value + label key for i18n)
#[derive(Debug, Clone)]
pub struct ProjectStat {
    pub value: &'static str,
    pub label_en: &'static str,
    pub label_fr: &'static str,
}

/// Full project data
#[derive(Debug, Clone)]
pub struct Project {
    pub id: &'static str,
    pub category: ProjectCategory,
    pub title_en: &'static str,
    pub title_fr: &'static str,
    pub date: &'static str,
    pub description_en: &'static str,
    pub description_fr: &'static str,
    pub tags: &'static [&'static str],
    pub tech_stack: &'static str,
    pub stats: &'static [ProjectStat],
}

impl Project {
    pub fn all_projects() -> Vec<&'static Project> {
        PROJECTS.iter().collect()
    }
}

// ============================================================================
// STATIC PROJECT DATA
// ============================================================================

static PROJECTS: &[Project] = &[
    // Personal Portfolio Website
    Project {
        id: "portfolio",
        category: ProjectCategory::Cloud,
        title_en: "Personal Portfolio & Blog Platform",
        title_fr: "Portfolio Personnel & Plateforme de Blog",
        date: "October 2025 - December 2025",
        description_en: "Built a full-stack portfolio website in Rust as a learning project, deliberately choosing the hard road over Next.js. Features include a theme system with 3 visual themes and 2 color modes (6 combinations), a build-time bilingual blog engine with syntax highlighting, SSR with WASM hydration for fast loads and SEO, and a contact form with honeypot traps and IP hashing. Security hardening includes CSP with nonces, HSTS, container lockdown (non-root, read-only filesystem, dropped capabilities), and Kubernetes network policies. Scored Grade A on external penetration testing. Deployed on K3s with Traefik ingress and Let's Encrypt certificates.",
        description_fr: "Construction d'un site portfolio full-stack en Rust comme projet d'apprentissage, choisissant délibérément la route difficile plutôt que Next.js. Les fonctionnalités incluent un système de thèmes avec 3 thèmes visuels et 2 modes de couleur (6 combinaisons), un moteur de blog bilingue compilé à la construction avec coloration syntaxique, du SSR avec hydratation WASM pour des chargements rapides et le SEO, et un formulaire de contact avec pièges à bots et hashage d'IP. Le durcissement sécuritaire inclut CSP avec nonces, HSTS, verrouillage du conteneur (non-root, système de fichiers en lecture seule, capabilities supprimées), et politiques réseau Kubernetes. Score Grade A au test de pénétration externe. Déployé sur K3s avec ingress Traefik et certificats Let's Encrypt.",
        tags: &["Rust", "Leptos", "Kubernetes", "Security"],
        tech_stack: "Rust, Leptos, Axum, PostgreSQL, K3s, Traefik, Podman, SCSS",
        stats: &[
            ProjectStat { value: "Grade A", label_en: "Security Score", label_fr: "Score Sécurité" },
            ProjectStat { value: "6", label_en: "Theme Combos", label_fr: "Combos de Thèmes" },
            ProjectStat { value: "SSR+WASM", label_en: "Rendering", label_fr: "Rendu" },
        ],
    },
    // Enterprise IT Asset Inventory System
    Project {
        id: "inventory",
        category: ProjectCategory::Automation,
        title_en: "Enterprise-Wide IT Asset Inventory System Implementation",
        title_fr: "Implémentation d'un Système d'Inventaire des Actifs IT à l'Échelle de l'Entreprise",
        date: "October 2023 - February 2024",
        description_en: "Led the implementation of a comprehensive IT inventory system, addressing critical gaps in infrastructure documentation and asset management. Developed automated PowerShell scripts for data collection across distributed endpoints without Active Directory, achieving 98% coverage of all IT assets. Established a centralized knowledge base using Excel and Jira Service Management, reducing information retrieval time from 48 hours to 30 minutes. This strategic initiative enabled data-driven decision-making for infrastructure investments and provided the commercial team with instant access to technical specifications. The project laid the foundation for subsequent security and cloud migration initiatives by providing complete visibility into the organization's IT landscape.",
        description_fr: "Direction de l'implémentation d'un système d'inventaire IT complet, comblant les lacunes critiques dans la documentation de l'infrastructure et la gestion des actifs. Développement de scripts PowerShell automatisés pour la collecte de données sur des endpoints distribués sans Active Directory, atteignant 98% de couverture de tous les actifs IT. Établissement d'une base de connaissances centralisée utilisant Excel et Jira Service Management, réduisant le temps de récupération d'information de 48 heures à 30 minutes. Cette initiative stratégique a permis une prise de décision basée sur les données pour les investissements d'infrastructure et a fourni à l'équipe commerciale un accès instantané aux spécifications techniques. Le projet a posé les fondations pour les initiatives ultérieures de sécurité et de migration cloud en fournissant une visibilité complète sur le paysage IT de l'organisation.",
        tags: &["PowerShell", "Automation", "ITSM", "Asset Management"],
        tech_stack: "PowerShell, Microsoft Excel, Google Sheets, Jira Service Management, Asset Management, Data Analytics",
        stats: &[
            ProjectStat { value: "98%", label_en: "Asset Coverage", label_fr: "Couverture des Actifs" },
            ProjectStat { value: "96%", label_en: "Time Saved", label_fr: "Temps Économisé" },
        ],
    },
    // Microsoft Intune Proof of Concept
    Project {
        id: "intune_poc",
        category: ProjectCategory::Automation,
        title_en: "Microsoft Intune Proof of Concept Implementation",
        title_fr: "Implémentation de Preuve de Concept Microsoft Intune",
        date: "September 2021 - July 2022",
        description_en: "Designed and deployed a comprehensive Microsoft Intune proof of concept environment, establishing an isolated testing framework for endpoint management capabilities. Configured multiple test devices across Windows 10/11 versions, Android, and iOS platforms to simulate real-world infrastructure diversity. Implemented compliance policies and Role-Based Access Control (RBAC) with Azure AD integration, ensuring secure identity management. Developed comprehensive documentation and standardized procedures, delivering a turnkey solution that enabled the organization to safely evaluate pre-production features. This strategic initiative provided critical insights for future enterprise-wide endpoint management deployment, minimizing implementation risks while maximizing feature adoption potential.",
        description_fr: "Conception et déploiement d'un environnement de preuve de concept Microsoft Intune complet, établissant un framework de test isolé pour les capacités de gestion des endpoints. Configuration de plusieurs appareils de test sur les versions Windows 10/11, Android et iOS pour simuler la diversité d'infrastructure du monde réel. Implémentation de politiques de conformité et de contrôle d'accès basé sur les rôles (RBAC) avec intégration Azure AD, assurant une gestion sécurisée des identités. Développement d'une documentation complète et de procédures standardisées, livrant une solution clé en main permettant à l'organisation d'évaluer en toute sécurité les fonctionnalités pré-production. Cette initiative stratégique a fourni des informations critiques pour le futur déploiement de gestion des endpoints à l'échelle de l'entreprise, minimisant les risques d'implémentation tout en maximisant le potentiel d'adoption des fonctionnalités.",
        tags: &["Intune", "Azure AD", "MDM", "RBAC"],
        tech_stack: "Microsoft Intune, Azure Active Directory, Windows 10/11, MDM, Compliance Policies, RBAC, PowerShell",
        stats: &[
            ProjectStat { value: "3", label_en: "Platforms", label_fr: "Plateformes" },
            ProjectStat { value: "RBAC", label_en: "Access Control", label_fr: "Contrôle d'Accès" },
        ],
    },
];

// ============================================================================
// COMPONENTS
// ============================================================================

/// Projects page - main component
#[component]
pub fn ProjectsPage() -> impl IntoView {
    let i18n = use_i18n();

    // Filter state: None = "All", Some(category) = specific filter
    let selected_category = RwSignal::new(Option::<ProjectCategory>::None);

    // Derive filtered projects list
    let filtered_projects = Signal::derive(move || {
        Project::all_projects()
            .into_iter()
            .filter(|p| {
                selected_category
                    .get()
                    .is_none_or(|cat| p.category == cat)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="projects-page">
            <PageHeader i18n=i18n />
            <FilterTabs i18n=i18n selected=selected_category />
            <ProjectsGrid i18n=i18n projects=filtered_projects />
            <GithubSection i18n=i18n />
        </div>
    }
}

/// Page header with title and subtitle
#[component]
fn PageHeader(i18n: I18nContext) -> impl IntoView {
    view! {
        <header class="page-header">
            <h1>{move || i18n.t().proj_page_title}</h1>
            <p class="page-subtitle">{move || i18n.t().proj_page_subtitle}</p>
        </header>
    }
}

/// Filter tabs for category selection
#[component]
fn FilterTabs(i18n: I18nContext, selected: RwSignal<Option<ProjectCategory>>) -> impl IntoView {
    view! {
        <div class="filter-tabs">
            <button
                class=move || if selected.get().is_none() { "filter-tab active" } else { "filter-tab" }
                on:click=move |_| selected.set(None)
            >
                {move || i18n.t().proj_filter_all}
            </button>
            {ProjectCategory::all().iter().map(|cat| {
                let cat = *cat;
                let label = move || match cat {
                    ProjectCategory::Cloud => i18n.t().proj_filter_cloud,
                    ProjectCategory::Security => i18n.t().proj_filter_security,
                    ProjectCategory::Automation => i18n.t().proj_filter_automation,
                };
                view! {
                    <button
                        class=move || if selected.get() == Some(cat) { "filter-tab active" } else { "filter-tab" }
                        on:click=move |_| selected.set(Some(cat))
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

/// Projects grid displaying filtered project cards
#[component]
fn ProjectsGrid(i18n: I18nContext, projects: Signal<Vec<&'static Project>>) -> impl IntoView {
    view! {
        <div class="projects-grid">
            <For
                each=move || projects.get().into_iter().enumerate()
                key=|(_, p)| p.id
                children=move |(index, project)| {
                    let delay = format!("animation-delay: {}s", index as f32 * 0.1);
                    view! {
                        <ProjectCard project=project i18n=i18n style=delay />
                    }
                }
            />
        </div>
    }
}

/// Individual project card
#[component]
fn ProjectCard(project: &'static Project, i18n: I18nContext, style: String) -> impl IntoView {
    view! {
        <article class="project-card" style=style>
            <div class="project-header">
                <h3 class="project-title">
                    {move || if i18n.is_french() { project.title_fr } else { project.title_en }}
                </h3>
                <p class="project-date">{project.date}</p>
                <div class="project-tags">
                    {project.tags.iter().map(|tag| view! {
                        <span class="tag">{*tag}</span>
                    }).collect_view()}
                </div>
            </div>
            <div class="project-body">
                <p class="project-description">
                    {move || if i18n.is_french() { project.description_fr } else { project.description_en }}
                </p>
                <div class="project-stats">
                    {project.stats.iter().map(|stat| view! {
                        <div class="stat">
                            <div class="stat-value">{stat.value}</div>
                            <div class="stat-label">
                                {move || if i18n.is_french() { stat.label_fr } else { stat.label_en }}
                            </div>
                        </div>
                    }).collect_view()}
                </div>
                <div class="project-tech">
                    <p class="tech-stack">
                        <span class="tech-label">{move || i18n.t().proj_tech_label}</span>
                        " "
                        {project.tech_stack}
                    </p>
                </div>
            </div>
        </article>
    }
}

// ============================================================================
// GITHUB SECTION
// ============================================================================

/// GitHub section showing public repositories
#[component]
fn GithubSection(i18n: I18nContext) -> impl IntoView {
    // Create resource for async data fetching
    let repos_resource = Resource::new(|| (), |_| get_github_repos());

    view! {
        <section class="github-section">
            <header class="section-header">
                <h2>{move || i18n.t().github_section_title}</h2>
                <p class="section-subtitle">{move || i18n.t().github_section_subtitle}</p>
            </header>

            <Suspense fallback=move || view! {
                <div class="github-loading">
                    <span class="loading-spinner"></span>
                    <span>{move || i18n.t().github_loading}</span>
                </div>
            }>
                {move || {
                    repos_resource.get().map(|result| {
                        match result {
                            Ok(data) => {
                                if data.repos.is_empty() {
                                    view! {
                                        <div class="github-empty">
                                            {move || i18n.t().github_no_repos}
                                        </div>
                                    }.into_any()
                                } else {
                                    let repos = data.repos.clone();
                                    view! {
                                        <div class="github-grid">
                                            <For
                                                each=move || repos.clone()
                                                key=|repo| repo.html_url.clone()
                                                children=move |repo| {
                                                    view! { <GithubRepoCard repo=repo i18n=i18n /> }
                                                }
                                            />
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(_) => view! {
                                <div class="github-error">
                                    {move || i18n.t().github_error}
                                </div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </section>
    }
}

/// Individual GitHub repository card
#[component]
fn GithubRepoCard(repo: GithubRepoDisplay, i18n: I18nContext) -> impl IntoView {
    let language_class = repo
        .language
        .as_ref()
        .map(|l| format!("lang-{}", l.to_lowercase().replace(' ', "-")))
        .unwrap_or_default();

    let html_url = repo.html_url.clone();
    let html_url_footer = repo.html_url.clone();

    view! {
        <article class="github-card">
            <div class="github-card-header">
                <h3 class="repo-name">
                    <a href=html_url target="_blank" rel="noopener noreferrer">
                        {repo.name.clone()}
                    </a>
                </h3>
                {repo.language.as_ref().map(|lang| view! {
                    <span class=format!("repo-language {}", language_class)>
                        {lang.clone()}
                    </span>
                })}
            </div>

            {repo.description.as_ref().map(|desc| view! {
                <p class="repo-description">{desc.clone()}</p>
            })}

            <div class="repo-meta">
                <span class="repo-stat">
                    <span class="stat-icon">"★"</span>
                    <span class="stat-value">{repo.stars}</span>
                    <span class="stat-label">{move || i18n.t().github_stars}</span>
                </span>
                <span class="repo-stat">
                    <span class="stat-icon">"⑂"</span>
                    <span class="stat-value">{repo.forks}</span>
                    <span class="stat-label">{move || i18n.t().github_forks}</span>
                </span>
            </div>

            {(!repo.topics.is_empty()).then(|| {
                let topics = repo.topics.clone();
                view! {
                    <div class="repo-topics">
                        {topics.into_iter().take(5).map(|topic| view! {
                            <span class="topic-tag">{topic}</span>
                        }).collect_view()}
                    </div>
                }
            })}

            <div class="repo-footer">
                {repo.updated_at.as_ref().map(|date| {
                    let date = date.clone();
                    view! {
                        <span class="repo-updated">
                            {move || i18n.t().github_updated}": "{date.clone()}
                        </span>
                    }
                })}
                <a href=html_url_footer target="_blank" rel="noopener noreferrer" class="github-link">
                    {move || i18n.t().github_view_on_github}
                    <span>" →"</span>
                </a>
            </div>
        </article>
    }
}
