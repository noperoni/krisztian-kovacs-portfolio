use super::Language;

/// All translatable strings in the application
#[derive(Debug, Clone)]
pub struct Translations {
    // Navigation
    pub nav_home: &'static str,
    pub nav_about: &'static str,
    pub nav_cv: &'static str,
    pub nav_projects: &'static str,
    pub nav_blog: &'static str,

    // Hero section
    pub hero_title: &'static str,
    pub hero_subtitle: &'static str,
    pub hero_description: &'static str,
    pub hero_cta_projects: &'static str,
    pub hero_cta_cv: &'static str,

    // Features section
    pub feature_cloud_title: &'static str,
    pub feature_cloud_desc: &'static str,
    pub feature_security_title: &'static str,
    pub feature_security_desc: &'static str,
    pub feature_ai_title: &'static str,
    pub feature_ai_desc: &'static str,

    // About page - Header
    pub about_page_title: &'static str,
    pub about_page_subtitle: &'static str,

    // About page - Profile
    pub about_profile_title: &'static str,

    // About page - Introduction
    pub about_intro_title: &'static str,
    pub about_intro_p1: &'static str,
    pub about_intro_p2: &'static str,
    pub about_intro_p3: &'static str,

    // About page - Skills (titles only, technical skills stay English)
    pub about_skills_core_title: &'static str,
    pub about_skills_interests_title: &'static str,

    // About page - Journey Timeline
    pub about_journey_title: &'static str,
    pub about_timeline_nomadia: &'static str,
    pub about_timeline_dynamips: &'static str,
    pub about_timeline_charier: &'static str,
    pub about_timeline_education_title: &'static str,
    pub about_timeline_education: &'static str,

    // About page - Contact CTA
    pub about_cta_title: &'static str,
    pub about_cta_description: &'static str,
    pub about_cta_button: &'static str,

    // Contact
    pub contact_title: &'static str,
    pub contact_name: &'static str,
    pub contact_email: &'static str,
    pub contact_message: &'static str,
    pub contact_send: &'static str,

    // CV Page - Hero & Summary
    pub cv_subtitle: &'static str,
    pub cv_summary_title: &'static str,
    pub cv_summary_text: &'static str,

    // CV Page - Technical Expertise
    pub cv_expertise_title: &'static str,
    pub cv_expertise_infra_title: &'static str,
    pub cv_expertise_support_title: &'static str,
    pub cv_expertise_tools_title: &'static str,
    pub cv_skill_support_levels: &'static str,
    pub cv_skill_audit: &'static str,
    pub cv_skill_project_mgmt: &'static str,

    // CV Page - Experience
    pub cv_experience_title: &'static str,
    pub cv_exp_nomadia_title: &'static str,
    pub cv_exp_nomadia_date: &'static str,
    pub cv_exp_nomadia_r1: &'static str,
    pub cv_exp_nomadia_r2: &'static str,
    pub cv_exp_nomadia_r3: &'static str,
    pub cv_exp_nomadia_r4: &'static str,
    pub cv_exp_dynamips_title: &'static str,
    pub cv_exp_dynamips_date: &'static str,
    pub cv_exp_dynamips_r1: &'static str,
    pub cv_exp_dynamips_r2: &'static str,
    pub cv_exp_dynamips_r3: &'static str,
    pub cv_exp_dynamips_r4: &'static str,
    pub cv_exp_charier_title: &'static str,
    pub cv_exp_charier_date: &'static str,
    pub cv_exp_charier_r1: &'static str,
    pub cv_exp_charier_r2: &'static str,
    pub cv_exp_charier_r3: &'static str,
    pub cv_exp_charier_r4: &'static str,

    // CV Page - Projects
    pub cv_projects_title: &'static str,
    pub cv_proj_k8s_title: &'static str,
    pub cv_proj_k8s_desc: &'static str,
    pub cv_proj_modernization_title: &'static str,
    pub cv_proj_modernization_desc: &'static str,
    pub cv_proj_endpoint_title: &'static str,
    pub cv_proj_endpoint_desc: &'static str,
    pub cv_proj_multicloud_title: &'static str,
    pub cv_proj_multicloud_desc: &'static str,

    // CV Page - Education
    pub cv_education_title: &'static str,
    pub cv_education_heading: &'static str,
    pub cv_edu_master_title: &'static str,
    pub cv_edu_master_level: &'static str,
    pub cv_edu_bachelor_title: &'static str,
    pub cv_edu_bachelor_level: &'static str,
    pub cv_certifications_heading: &'static str,

    // CV Page - Languages
    pub cv_languages_title: &'static str,
    pub cv_lang_english: &'static str,
    pub cv_lang_french: &'static str,
    pub cv_lang_hungarian: &'static str,
    pub cv_lang_romanian: &'static str,
    pub cv_lang_german: &'static str,
    pub cv_lang_native: &'static str,
    pub cv_lang_professional: &'static str,
    pub cv_lang_limited: &'static str,

    // Projects Page
    pub proj_page_title: &'static str,
    pub proj_page_subtitle: &'static str,
    pub proj_filter_all: &'static str,
    pub proj_filter_cloud: &'static str,
    pub proj_filter_security: &'static str,
    pub proj_filter_automation: &'static str,
    pub proj_featured_badge: &'static str,
    pub proj_tech_label: &'static str,

    // Common
    pub loading: &'static str,
    pub error: &'static str,
    pub success: &'static str,
}

impl Translations {
    /// Get translations for a specific language
    pub fn for_language(lang: Language) -> Self {
        match lang {
            Language::En => Self::english(),
            Language::Fr => Self::french(),
        }
    }

    fn english() -> Self {
        Self {
            // Navigation
            nav_home: "Home",
            nav_about: "About",
            nav_cv: "CV",
            nav_projects: "Projects",
            nav_blog: "Blog",

            // Hero section
            hero_title: "Krisztián Kovács",
            hero_subtitle: "IT Infrastructure Engineer & Project Manager",
            hero_description: "Crafting robust cloud solutions and leading cross-functional teams to deliver enterprise-grade infrastructure. Specializing in cloud security, Kubernetes orchestration, and regulatory compliance.",
            hero_cta_projects: "View Projects",
            hero_cta_cv: "Download CV",

            // Features section
            feature_cloud_title: "Cloud Architecture",
            feature_cloud_desc: "Designing and implementing scalable cloud solutions on Azure, AWS, and GCP with focus on performance and cost optimization.",
            feature_security_title: "Security & Compliance",
            feature_security_desc: "Ensuring GDPR compliance, implementing ISO 27001 standards, and maintaining robust security postures across infrastructures.",
            feature_ai_title: "AI & Automation",
            feature_ai_desc: "Leveraging ML/AI for infrastructure optimization and implementing automation with Ansible, Terraform, and PowerShell.",

            // About page - Header
            about_page_title: "About Me",
            about_page_subtitle: "IT Engineer, Problem Solver, Innovation Enthusiast",

            // About page - Profile
            about_profile_title: "IT Infrastructure Engineer & Project Manager based in France",

            // About page - Introduction
            about_intro_title: "Hello, I'm Krisztián",
            about_intro_p1: "I'm an IT Infrastructure Engineer with a passion for building robust, scalable systems that drive business value. Currently working as an IT Expert at Nomadia in Nantes, France, I specialize in cloud architecture, regulatory compliance, and cross-departmental project management.",
            about_intro_p2: "With expertise spanning from Level 3 support to strategic IT implementations, I thrive on solving complex technical challenges and transforming infrastructure to meet modern demands. My approach combines technical excellence with a keen understanding of business needs, ensuring solutions that are both innovative and practical.",
            about_intro_p3: "Beyond the technical realm, I'm multilingual (speaking English, French, Hungarian, Romanian, and some German), which has proven invaluable in managing international projects and collaborating with diverse teams across Europe.",

            // About page - Skills
            about_skills_core_title: "Core Expertise",
            about_skills_interests_title: "Current Interests",

            // About page - Journey Timeline
            about_journey_title: "My Journey",
            about_timeline_nomadia: "Leading strategic IT projects, conducting infrastructure audits, and driving innovation through ML/AI integration.",
            about_timeline_dynamips: "Managed cloud infrastructure, modernized legacy systems, and ensured GDPR compliance across platforms.",
            about_timeline_charier: "Started my professional journey implementing Intune solutions and supporting enterprise mobility initiatives.",
            about_timeline_education_title: "Continuous Learning",
            about_timeline_education: "Pursuing Master's degree at EPSI while gaining hands-on experience through work-study programs.",

            // About page - Contact CTA
            about_cta_title: "Let's Connect",
            about_cta_description: "I'm always interested in discussing new technologies, challenging projects, or potential collaborations.",
            about_cta_button: "Get In Touch",

            // Contact
            contact_title: "Get in Touch",
            contact_name: "Name",
            contact_email: "Email",
            contact_message: "Message",
            contact_send: "Send Message",

            // CV Page - Hero & Summary
            cv_subtitle: "IT Infrastructure Engineer | Inter-Departmental IT Project Manager",
            cv_summary_title: "Professional Summary",
            cv_summary_text: "IT Engineer with expertise in enterprise system optimization, cloud security, and multi-service project management. Currently IT Expert at Nomadia, specializing in IT audit, regulatory compliance, and inter-departmental IT project management. Proven track record in Level 3 IT Support, complex incident management, and strategic IT implementation.",

            // CV Page - Technical Expertise
            cv_expertise_title: "Technical Expertise",
            cv_expertise_infra_title: "Infrastructure & Virtualization",
            cv_expertise_support_title: "IT Support & Management",
            cv_expertise_tools_title: "Technologies & Tools",
            cv_skill_support_levels: "Level 1-3 Support",
            cv_skill_audit: "IT Audit & Compliance",
            cv_skill_project_mgmt: "Project Management",

            // CV Page - Experience
            cv_experience_title: "Professional Experience",
            cv_exp_nomadia_title: "IT Expert (Work-Study Program)",
            cv_exp_nomadia_date: "Oct 2023 - Present",
            cv_exp_nomadia_r1: "Strategic IT project management and implementation across multiple departments",
            cv_exp_nomadia_r2: "IT infrastructure audit and regulatory compliance (GDPR, ISO 27001)",
            cv_exp_nomadia_r3: "Level 3 IT Support and complex incident resolution",
            cv_exp_nomadia_r4: "Active participation in internal R&D on ML & AI integration",
            cv_exp_dynamips_title: "IT Administrator (Work-Study Program)",
            cv_exp_dynamips_date: "Sep 2022 - Sep 2023",
            cv_exp_dynamips_r1: "M365, Network, AD, RDS, and Azure incident resolution",
            cv_exp_dynamips_r2: "VEEAM administration and optimization",
            cv_exp_dynamips_r3: "RDS migration project participation",
            cv_exp_dynamips_r4: "Legacy infrastructure modernization",
            cv_exp_charier_title: "IT Technician (Work-Study Program)",
            cv_exp_charier_date: "Sep 2020 - Sep 2022",
            cv_exp_charier_r1: "Workstation preparation and configuration",
            cv_exp_charier_r2: "End-user IT support",
            cv_exp_charier_r3: "Intune project management for mobility solutions",
            cv_exp_charier_r4: "IT park maintenance",

            // CV Page - Projects
            cv_projects_title: "Key Projects",
            cv_proj_k8s_title: "Cloud-Native Kubernetes Infrastructure",
            cv_proj_k8s_desc: "Delivered production-ready Kubernetes infrastructure on GCP with 99.95% uptime and 28% under budget.",
            cv_proj_modernization_title: "IT Modernization & International Integration",
            cv_proj_modernization_desc: "Led IT transformation implementing ITIL and ISO 27001, reducing order processing by 25%.",
            cv_proj_endpoint_title: "Enterprise Endpoint Management",
            cv_proj_endpoint_desc: "Architected Intune solution with 98% BitLocker coverage, reducing deployment time from days to hours.",
            cv_proj_multicloud_title: "Multi-Cloud Infrastructure Transformation",
            cv_proj_multicloud_desc: "Migrated OVH to Azure achieving 62% latency reduction and 2-4h provisioning time.",

            // CV Page - Education
            cv_education_title: "Education & Certifications",
            cv_education_heading: "Education",
            cv_edu_master_title: "Expert in IT and Information Systems",
            cv_edu_master_level: "Master's Level (RNCP Level 7)",
            cv_edu_bachelor_title: "Secure Infrastructure Administrator",
            cv_edu_bachelor_level: "Bachelor's Level (RNCP Level 6)",
            cv_certifications_heading: "Certifications",

            // CV Page - Languages
            cv_languages_title: "Languages",
            cv_lang_english: "English",
            cv_lang_french: "French",
            cv_lang_hungarian: "Hungarian",
            cv_lang_romanian: "Romanian",
            cv_lang_german: "German",
            cv_lang_native: "Native or bilingual",
            cv_lang_professional: "Full professional",
            cv_lang_limited: "Limited working",

            // Projects Page
            proj_page_title: "Projects Portfolio",
            proj_page_subtitle: "A showcase of my technical projects and achievements",
            proj_filter_all: "All Projects",
            proj_filter_cloud: "Cloud & Infrastructure",
            proj_filter_security: "Security & Compliance",
            proj_filter_automation: "Automation & DevOps",
            proj_featured_badge: "Featured",
            proj_tech_label: "Tech:",

            // Common
            loading: "Loading...",
            error: "An error occurred",
            success: "Success!",
        }
    }

    fn french() -> Self {
        Self {
            // Navigation
            nav_home: "Accueil",
            nav_about: "À propos",
            nav_cv: "CV",
            nav_projects: "Projets",
            nav_blog: "Blog",

            // Hero section
            hero_title: "Krisztián Kovács",
            hero_subtitle: "Ingénieur Infrastructure IT & Chef de Projet",
            hero_description: "Conception de solutions cloud robustes et direction d'équipes interfonctionnelles pour livrer des infrastructures d'entreprise. Spécialisé en sécurité cloud, orchestration Kubernetes et conformité réglementaire.",
            hero_cta_projects: "Voir Projets",
            hero_cta_cv: "Télécharger CV",

            // Features section
            feature_cloud_title: "Architecture Cloud",
            feature_cloud_desc: "Conception et implémentation de solutions cloud évolutives sur Azure, AWS et GCP avec focus sur la performance et l'optimisation des coûts.",
            feature_security_title: "Sécurité & Conformité",
            feature_security_desc: "Assurer la conformité RGPD, implémenter les normes ISO 27001 et maintenir des postures de sécurité robustes sur les infrastructures.",
            feature_ai_title: "IA & Automatisation",
            feature_ai_desc: "Exploitation du ML/IA pour l'optimisation des infrastructures et implémentation de l'automatisation avec Ansible, Terraform et PowerShell.",

            // About page - Header
            about_page_title: "À Propos de Moi",
            about_page_subtitle: "Ingénieur IT, Résolveur de Problèmes, Passionné d'Innovation",

            // About page - Profile
            about_profile_title: "Ingénieur Infrastructure IT & Chef de Projet basé en France",

            // About page - Introduction
            about_intro_title: "Bonjour, je suis Krisztián",
            about_intro_p1: "Je suis un Ingénieur Infrastructure IT passionné par la construction de systèmes robustes et évolutifs qui génèrent de la valeur commerciale. Actuellement Expert IT chez Nomadia à Nantes, France, je me spécialise dans l'architecture cloud, la conformité réglementaire et la gestion de projets inter-départementaux.",
            about_intro_p2: "Avec une expertise allant du support de niveau 3 aux implémentations IT stratégiques, je m'épanouis dans la résolution de défis techniques complexes et la transformation d'infrastructures pour répondre aux exigences modernes. Mon approche combine l'excellence technique avec une compréhension approfondie des besoins commerciaux, garantissant des solutions à la fois innovantes et pratiques.",
            about_intro_p3: "Au-delà du domaine technique, je suis multilingue (parlant anglais, français, hongrois, roumain et un peu d'allemand), ce qui s'est avéré inestimable pour gérer des projets internationaux et collaborer avec des équipes diverses à travers l'Europe.",

            // About page - Skills
            about_skills_core_title: "Expertise Principale",
            about_skills_interests_title: "Intérêts Actuels",

            // About page - Journey Timeline
            about_journey_title: "Mon Parcours",
            about_timeline_nomadia: "Direction de projets IT stratégiques, réalisation d'audits d'infrastructure et stimulation de l'innovation par l'intégration ML/IA.",
            about_timeline_dynamips: "Gestion de l'infrastructure cloud, modernisation des systèmes legacy et assurance de la conformité RGPD sur toutes les plateformes.",
            about_timeline_charier: "Début de mon parcours professionnel en implémentant des solutions Intune et en soutenant les initiatives de mobilité d'entreprise.",
            about_timeline_education_title: "Apprentissage Continu",
            about_timeline_education: "Poursuite d'un Master à l'EPSI tout en acquérant une expérience pratique grâce aux programmes d'alternance.",

            // About page - Contact CTA
            about_cta_title: "Connectons-nous",
            about_cta_description: "Je suis toujours intéressé pour discuter de nouvelles technologies, de projets stimulants ou de collaborations potentielles.",
            about_cta_button: "Contactez-moi",

            // Contact
            contact_title: "Me Contacter",
            contact_name: "Nom",
            contact_email: "Email",
            contact_message: "Message",
            contact_send: "Envoyer",

            // CV Page - Hero & Summary
            cv_subtitle: "Ingénieur Infrastructure IT | Chef de Projet IT Inter-Départemental",
            cv_summary_title: "Résumé Professionnel",
            cv_summary_text: "Ingénieur IT spécialisé dans l'optimisation des systèmes d'entreprise, la sécurité cloud et la gestion de projets multi-services. Actuellement Expert IT chez Nomadia, spécialisé dans l'audit IT, la conformité réglementaire et la gestion de projets IT inter-départementaux. Expertise confirmée en support IT Niveau 3, gestion d'incidents complexes et implémentation stratégique IT.",

            // CV Page - Technical Expertise
            cv_expertise_title: "Expertise Technique",
            cv_expertise_infra_title: "Infrastructure & Virtualisation",
            cv_expertise_support_title: "Support & Gestion IT",
            cv_expertise_tools_title: "Technologies & Outils",
            cv_skill_support_levels: "Support Niveaux 1-3",
            cv_skill_audit: "Audit IT & Conformité",
            cv_skill_project_mgmt: "Gestion de Projet",

            // CV Page - Experience
            cv_experience_title: "Expérience Professionnelle",
            cv_exp_nomadia_title: "Expert IT (Alternance)",
            cv_exp_nomadia_date: "Oct 2023 - Présent",
            cv_exp_nomadia_r1: "Gestion et implémentation de projets IT stratégiques multi-départements",
            cv_exp_nomadia_r2: "Audit d'infrastructure IT et conformité réglementaire (RGPD, ISO 27001)",
            cv_exp_nomadia_r3: "Support IT Niveau 3 et résolution d'incidents complexes",
            cv_exp_nomadia_r4: "Participation active à la R&D interne sur l'intégration ML & IA",
            cv_exp_dynamips_title: "Administrateur IT (Alternance)",
            cv_exp_dynamips_date: "Sep 2022 - Sep 2023",
            cv_exp_dynamips_r1: "Résolution d'incidents M365, Réseau, AD, RDS et Azure",
            cv_exp_dynamips_r2: "Administration et optimisation VEEAM",
            cv_exp_dynamips_r3: "Participation au projet de migration RDS",
            cv_exp_dynamips_r4: "Modernisation d'infrastructure legacy",
            cv_exp_charier_title: "Technicien IT (Alternance)",
            cv_exp_charier_date: "Sep 2020 - Sep 2022",
            cv_exp_charier_r1: "Préparation et configuration de postes de travail",
            cv_exp_charier_r2: "Support IT utilisateur final",
            cv_exp_charier_r3: "Gestion de projet Intune pour solutions de mobilité",
            cv_exp_charier_r4: "Maintenance du parc informatique",

            // CV Page - Projects
            cv_projects_title: "Projets Clés",
            cv_proj_k8s_title: "Infrastructure Kubernetes Cloud-Native",
            cv_proj_k8s_desc: "Livraison d'infrastructure Kubernetes production sur GCP avec 99,95% de disponibilité et 28% sous budget.",
            cv_proj_modernization_title: "Modernisation IT & Intégration Internationale",
            cv_proj_modernization_desc: "Direction de transformation IT avec ITIL et ISO 27001, réduction de 25% du temps de traitement.",
            cv_proj_endpoint_title: "Gestion des Endpoints d'Entreprise",
            cv_proj_endpoint_desc: "Architecture Intune avec 98% de chiffrement BitLocker, réduction du déploiement de jours à heures.",
            cv_proj_multicloud_title: "Transformation Infrastructure Multi-Cloud",
            cv_proj_multicloud_desc: "Migration OVH vers Azure avec réduction latence de 62% et provisionnement en 2-4h.",

            // CV Page - Education
            cv_education_title: "Formation & Certifications",
            cv_education_heading: "Formation",
            cv_edu_master_title: "Expert en Informatique et SI",
            cv_edu_master_level: "Niveau Master (RNCP Niveau 7)",
            cv_edu_bachelor_title: "Administrateur d'Infrastructures Sécurisées",
            cv_edu_bachelor_level: "Niveau Licence (RNCP Niveau 6)",
            cv_certifications_heading: "Certifications",

            // CV Page - Languages
            cv_languages_title: "Langues",
            cv_lang_english: "Anglais",
            cv_lang_french: "Français",
            cv_lang_hungarian: "Hongrois",
            cv_lang_romanian: "Roumain",
            cv_lang_german: "Allemand",
            cv_lang_native: "Natif ou bilingue",
            cv_lang_professional: "Compétence professionnelle complète",
            cv_lang_limited: "Compétence professionnelle limitée",

            // Projects Page
            proj_page_title: "Portfolio de Projets",
            proj_page_subtitle: "Une vitrine de mes projets techniques et réalisations",
            proj_filter_all: "Tous les Projets",
            proj_filter_cloud: "Cloud & Infrastructure",
            proj_filter_security: "Sécurité & Conformité",
            proj_filter_automation: "Automatisation & DevOps",
            proj_featured_badge: "En Vedette",
            proj_tech_label: "Tech :",

            // Common
            loading: "Chargement...",
            error: "Une erreur s'est produite",
            success: "Succès !",
        }
    }
}
