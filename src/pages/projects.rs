use leptos::prelude::*;

use crate::i18n::{use_i18n, I18nContext};

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

    pub fn id(&self) -> &'static str {
        match self {
            ProjectCategory::Cloud => "cloud",
            ProjectCategory::Security => "security",
            ProjectCategory::Automation => "automation",
        }
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
    pub is_featured: bool,
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
    // Featured: Cloud-Native Kubernetes Infrastructure
    Project {
        id: "k8s",
        category: ProjectCategory::Cloud,
        title_en: "Cloud-Native Kubernetes Infrastructure",
        title_fr: "Infrastructure Kubernetes Native Cloud",
        date: "April 2025 - July 2025",
        description_en: "Delivered a production-ready Kubernetes infrastructure on Google Cloud Platform for an enterprise client. Led a multicultural team implementing Infrastructure as Code with Terraform, comprehensive monitoring with Prometheus/Grafana, and automated CI/CD pipelines.",
        description_fr: "Livraison d'une infrastructure Kubernetes prête pour la production sur Google Cloud Platform pour un client d'entreprise. Direction d'une équipe multiculturelle implémentant l'Infrastructure as Code avec Terraform, une surveillance complète avec Prometheus/Grafana et des pipelines CI/CD automatisés.",
        tags: &["Kubernetes", "GCP", "Terraform", "Production"],
        tech_stack: "Kubernetes (GKE), Terraform, Ansible, Prometheus, Grafana, Docker, GitOps",
        stats: &[
            ProjectStat { value: "99.95%", label_en: "Uptime Achieved", label_fr: "Disponibilité Atteinte" },
            ProjectStat { value: "28%", label_en: "Under Budget", label_fr: "Sous Budget" },
            ProjectStat { value: "62%", label_en: "ROI Projection", label_fr: "Projection ROI" },
            ProjectStat { value: "4", label_en: "Team Members", label_fr: "Membres d'Équipe" },
        ],
        is_featured: true,
    },
    // Enterprise Endpoint Management Platform
    Project {
        id: "endpoint",
        category: ProjectCategory::Security,
        title_en: "Enterprise Endpoint Management Platform",
        title_fr: "Plateforme de Gestion des Endpoints d'Entreprise",
        date: "March 2024 - October 2024",
        description_en: "Architected and deployed a comprehensive Microsoft Intune solution, transforming endpoint security across 30+ workstations. Achieved 98% BitLocker encryption coverage and established Azure AD integration with MFA and conditional access policies.",
        description_fr: "Architecture et déploiement d'une solution Microsoft Intune complète, transformant la sécurité des endpoints sur plus de 30 postes de travail. Réalisation d'une couverture de chiffrement BitLocker de 98% et établissement de l'intégration Azure AD avec MFA et politiques d'accès conditionnel.",
        tags: &["Microsoft Intune", "Azure AD", "Security"],
        tech_stack: "Microsoft Intune, Azure AD, BitLocker, PowerShell, Windows Autopilot",
        stats: &[
            ProjectStat { value: "92%", label_en: "Incident Reduction", label_fr: "Réduction des Incidents" },
            ProjectStat { value: "1 hour", label_en: "Deploy Time", label_fr: "Temps de Déploiement" },
        ],
        is_featured: false,
    },
    // Multi-Cloud Infrastructure Transformation
    Project {
        id: "multicloud",
        category: ProjectCategory::Cloud,
        title_en: "Multi-Cloud Infrastructure Transformation",
        title_fr: "Transformation d'Infrastructure Multi-Cloud",
        date: "October 2024",
        description_en: "Spearheaded strategic cloud transformation migrating infrastructure from OVH to Microsoft Azure. Designed scalable architecture supporting IaaS and PaaS services, reducing environment provisioning time from 3-5 days to 2-4 hours.",
        description_fr: "Direction de la transformation cloud stratégique migrant l'infrastructure d'OVH vers Microsoft Azure. Conception d'une architecture évolutive supportant les services IaaS et PaaS, réduisant le temps de provisionnement d'environnement de 3-5 jours à 2-4 heures.",
        tags: &["Azure", "Migration", "IaC"],
        tech_stack: "Azure, Azure Functions, Prometheus, Grafana, PowerShell, DevOps",
        stats: &[
            ProjectStat { value: "62%", label_en: "Latency Reduction", label_fr: "Réduction de Latence" },
            ProjectStat { value: "99.95%", label_en: "Availability", label_fr: "Disponibilité" },
        ],
        is_featured: false,
    },
    // IT Modernization & International Integration
    Project {
        id: "itil",
        category: ProjectCategory::Automation,
        title_en: "IT Modernization & International Integration",
        title_fr: "Modernisation IT & Intégration Internationale",
        date: "October 2024 - March 2025",
        description_en: "Led comprehensive IT transformation for electronics manufacturer integrating French and Spanish operations. Implemented ITIL framework and ISO 27001-compliant security policies, achieving 25% reduction in order processing time.",
        description_fr: "Direction de la transformation IT complète pour un fabricant d'électronique intégrant les opérations françaises et espagnoles. Implémentation du framework ITIL et des politiques de sécurité conformes ISO 27001, réalisant une réduction de 25% du temps de traitement des commandes.",
        tags: &["ITIL", "ISO 27001", "Lean IT"],
        tech_stack: "ITIL, ISO 27001, ERP Integration, Green IT, Lean Methodologies",
        stats: &[
            ProjectStat { value: "25%", label_en: "Time Reduction", label_fr: "Réduction du Temps" },
            ProjectStat { value: "20%", label_en: "Energy Savings", label_fr: "Économies d'Énergie" },
        ],
        is_featured: false,
    },
    // Enterprise IT Asset Inventory System
    Project {
        id: "inventory",
        category: ProjectCategory::Automation,
        title_en: "Enterprise IT Asset Inventory System",
        title_fr: "Système d'Inventaire des Actifs IT d'Entreprise",
        date: "October 2023 - February 2024",
        description_en: "Developed automated PowerShell scripts for comprehensive IT asset discovery across distributed endpoints. Established centralized knowledge base reducing information retrieval time from 48 hours to 30 minutes.",
        description_fr: "Développement de scripts PowerShell automatisés pour la découverte complète des actifs IT sur des endpoints distribués. Établissement d'une base de connaissances centralisée réduisant le temps de récupération d'information de 48 heures à 30 minutes.",
        tags: &["PowerShell", "Automation", "ITSM"],
        tech_stack: "PowerShell, Jira Service Management, Excel, Data Analytics",
        stats: &[
            ProjectStat { value: "98%", label_en: "Asset Coverage", label_fr: "Couverture des Actifs" },
            ProjectStat { value: "96%", label_en: "Time Saved", label_fr: "Temps Économisé" },
        ],
        is_featured: false,
    },
    // Enterprise Security & Backup Implementation
    Project {
        id: "security_backup",
        category: ProjectCategory::Security,
        title_en: "Enterprise Security & Backup Implementation",
        title_fr: "Implémentation de Sécurité et Sauvegarde d'Entreprise",
        date: "April 2023 - August 2023",
        description_en: "Deployed enterprise-grade security with SOPHOS XGS firewall and established robust backup infrastructure using VEEAM. Transformed access management implementing AGDLP methodology for enhanced security posture.",
        description_fr: "Déploiement de sécurité de niveau entreprise avec pare-feu SOPHOS XGS et établissement d'une infrastructure de sauvegarde robuste avec VEEAM. Transformation de la gestion des accès en implémentant la méthodologie AGDLP pour une posture de sécurité renforcée.",
        tags: &["Sophos", "VEEAM", "Security"],
        tech_stack: "SOPHOS XGS, VEEAM, Active Directory, ReFS, SSL VPN, PowerShell",
        stats: &[
            ProjectStat { value: "5 days", label_en: "Implementation", label_fr: "Implémentation" },
            ProjectStat { value: "100%", label_en: "Backup Coverage", label_fr: "Couverture de Sauvegarde" },
        ],
        is_featured: false,
    },
    // Big Data Analytics & Predictive Modeling
    Project {
        id: "bigdata",
        category: ProjectCategory::Cloud,
        title_en: "Big Data Analytics & Predictive Modeling",
        title_fr: "Analyse Big Data & Modélisation Prédictive",
        date: "October 2023 - March 2024",
        description_en: "Developed a comprehensive big data solution for electoral trend analysis using machine learning algorithms. Led a team of 5 to design and implement a predictive model achieving 85% accuracy.",
        description_fr: "Développement d'une solution big data complète pour l'analyse des tendances électorales utilisant des algorithmes de machine learning. Direction d'une équipe de 5 personnes pour concevoir et implémenter un modèle prédictif atteignant 85% de précision.",
        tags: &["Python", "Power BI", "Machine Learning"],
        tech_stack: "Python, Power BI, ETL Tools, Machine Learning, SQL, Data Visualization",
        stats: &[
            ProjectStat { value: "85%", label_en: "Model Accuracy", label_fr: "Précision du Modèle" },
            ProjectStat { value: "5", label_en: "Team Members", label_fr: "Membres de l'Équipe" },
        ],
        is_featured: false,
    },
    // Client IT Unification
    Project {
        id: "unification",
        category: ProjectCategory::Cloud,
        title_en: "Client IT Unification",
        title_fr: "Unification IT Client",
        date: "April 2024 - September 2024",
        description_en: "Architected and planned a comprehensive IT infrastructure unification strategy for a global sports organization with 700+ employees across 4 continents. Designed virtualization solutions using Proxmox and created disaster recovery plans.",
        description_fr: "Architecture et planification d'une stratégie complète d'unification d'infrastructure IT pour une organisation sportive mondiale avec 700+ employés sur 4 continents. Conception de solutions de virtualisation avec Proxmox et création de plans de reprise d'activité.",
        tags: &["Proxmox", "Active Directory", "Azure"],
        tech_stack: "Proxmox, Active Directory, VPN, pfSense, Zabbix, Microsoft Teams, Azure",
        stats: &[
            ProjectStat { value: "700+", label_en: "Employees", label_fr: "Employés" },
            ProjectStat { value: "20%", label_en: "Cost Reduction", label_fr: "Réduction des Coûts" },
        ],
        is_featured: false,
    },
    // IoT Database Architecture
    Project {
        id: "iot",
        category: ProjectCategory::Cloud,
        title_en: "IoT Database Architecture",
        title_fr: "Architecture de Base de Données IoT",
        date: "July 2023 - September 2023",
        description_en: "Designed and implemented a scalable database architecture for an IoT project \"ThunderStorm\". Created comprehensive data models to handle real-time sensor data, GPS tracking, and weather information.",
        description_fr: "Conception et implémentation d'une architecture de base de données évolutive pour un projet IoT \"ThunderStorm\". Création de modèles de données complets pour gérer les données de capteurs en temps réel, le suivi GPS et les informations météorologiques.",
        tags: &["Azure SQL", "IoT Hub", "Security"],
        tech_stack: "Azure SQL Database, IoT Hub, Stream Analytics, Data modeling, TLS encryption, Azure security",
        stats: &[
            ProjectStat { value: "Real-time", label_en: "Data Processing", label_fr: "Traitement des Données" },
            ProjectStat { value: "TLS", label_en: "Encryption", label_fr: "Chiffrement" },
        ],
        is_featured: false,
    },
    // Legacy System Modernization
    Project {
        id: "legacy",
        category: ProjectCategory::Automation,
        title_en: "Legacy System Modernization",
        title_fr: "Modernisation de Systèmes Legacy",
        date: "March 2023 - June 2023",
        description_en: "Successfully modernized legacy IT infrastructure through Windows Server 2008 to 2012 migration, implemented MDT/WDS for automated deployment, and established high availability for critical services.",
        description_fr: "Modernisation réussie d'infrastructure IT legacy par migration Windows Server 2008 vers 2012, implémentation MDT/WDS pour déploiement automatisé, et établissement de haute disponibilité pour services critiques.",
        tags: &["Windows Server", "MDT", "pfSense"],
        tech_stack: "Windows Server 2012, MDT, WDS, pfSense, GLPI, Active Directory, Samba, scripts Linux",
        stats: &[
            ProjectStat { value: "100%", label_en: "Uptime", label_fr: "Disponibilité" },
            ProjectStat { value: "Automated", label_en: "Deployment", label_fr: "Déploiement" },
        ],
        is_featured: false,
    },
    // Cloud Migration & GDPR Compliance
    Project {
        id: "rgpd",
        category: ProjectCategory::Security,
        title_en: "Cloud Migration & GDPR Compliance",
        title_fr: "Migration Cloud et Conformité RGPD",
        date: "December 2022 - February 2023",
        description_en: "Architected and planned comprehensive cloud migration strategy with focus on GDPR and HDS compliance. Designed Azure infrastructure to replace on-premises servers while ensuring data sovereignty and security.",
        description_fr: "Architecture et planification d'une stratégie complète de migration cloud, avec focus sur la conformité RGPD et HDS. Conception d'infrastructure Azure pour remplacer les serveurs on-premises tout en garantissant souveraineté et sécurité des données.",
        tags: &["Azure", "RGPD", "Compliance"],
        tech_stack: "Azure (SQL Database, VMs, Backup), VEEAM, Sophos, outils conformité RGPD, PowerShell",
        stats: &[
            ProjectStat { value: "1M€", label_en: "Budget Optimization", label_fr: "Optimisation Budget" },
            ProjectStat { value: "RGPD", label_en: "Compliant", label_fr: "Conforme" },
        ],
        is_featured: false,
    },
    // IT Infrastructure Audit & Configuration
    Project {
        id: "audit",
        category: ProjectCategory::Automation,
        title_en: "IT Infrastructure Audit & Configuration",
        title_fr: "Audit et Configuration d'Infrastructure IT",
        date: "September 2022 - November 2022",
        description_en: "Designed and deployed complete virtualized IT infrastructure including network architecture, Active Directory implementation and supervision systems. Led team of 4 to deliver enterprise infrastructure.",
        description_fr: "Conception et déploiement d'une infrastructure IT virtualisée complète incluant architecture réseau, implémentation Active Directory et systèmes de supervision. Direction d'une équipe de 4 personnes pour livrer une infrastructure d'entreprise.",
        tags: &["Proxmox", "Zabbix", "GLPI"],
        tech_stack: "Proxmox, Active Directory, VEEAM, Zabbix, GLPI, Cisco, RAID, PowerShell",
        stats: &[
            ProjectStat { value: "4", label_en: "Team Members", label_fr: "Membres d'Équipe" },
            ProjectStat { value: "HA", label_en: "High Availability", label_fr: "Haute Disponibilité" },
        ],
        is_featured: false,
    },
    // Microsoft Intune Proof of Concept
    Project {
        id: "intune_poc",
        category: ProjectCategory::Automation,
        title_en: "Microsoft Intune Proof of Concept",
        title_fr: "Preuve de Concept Microsoft Intune",
        date: "September 2021 - July 2022",
        description_en: "Designed and deployed comprehensive Microsoft Intune proof of concept environment. Configured multiple test devices across Windows 10/11, Android, and iOS platforms to simulate real-world diversity.",
        description_fr: "Conception et déploiement d'un environnement de preuve de concept Microsoft Intune complet. Configuration de plusieurs appareils de test sur Windows 10/11, Android et iOS pour simuler la diversité du monde réel.",
        tags: &["Intune", "Azure AD", "MDM"],
        tech_stack: "Microsoft Intune, Azure Active Directory, Windows 10/11, MDM, Compliance Policies, RBAC, PowerShell",
        stats: &[
            ProjectStat { value: "3", label_en: "Platforms", label_fr: "Plateformes" },
            ProjectStat { value: "RBAC", label_en: "Access Control", label_fr: "Contrôle d'Accès" },
        ],
        is_featured: false,
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

    // Derive filtered projects list (featured always first)
    let filtered_projects = Signal::derive(move || {
        let mut projects: Vec<&'static Project> = Project::all_projects()
            .into_iter()
            .filter(|p| {
                selected_category
                    .get()
                    .map_or(true, |cat| p.category == cat)
            })
            .collect();
        // Sort: featured first
        projects.sort_by(|a, b| b.is_featured.cmp(&a.is_featured));
        projects
    });

    view! {
        <div class="projects-page">
            <PageHeader i18n=i18n.clone() />
            <FilterTabs i18n=i18n.clone() selected=selected_category />
            <ProjectsGrid i18n=i18n projects=filtered_projects />
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
                        <ProjectCard project=project i18n=i18n.clone() style=delay />
                    }
                }
            />
        </div>
    }
}

/// Individual project card (handles both featured and regular)
#[component]
fn ProjectCard(project: &'static Project, i18n: I18nContext, style: String) -> impl IntoView {
    let card_class = if project.is_featured {
        "project-card featured-project"
    } else {
        "project-card"
    };

    view! {
        <article class=card_class style=style>
            {project.is_featured.then(|| view! {
                <span class="featured-badge">{move || i18n.t().proj_featured_badge}</span>
            })}
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
