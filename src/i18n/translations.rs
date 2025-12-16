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

            // Common
            loading: "Chargement...",
            error: "Une erreur s'est produite",
            success: "Succès !",
        }
    }
}
