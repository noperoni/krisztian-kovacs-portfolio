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

    // About page
    pub about_title: &'static str,
    pub about_intro: &'static str,

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

            // About page
            about_title: "About Me",
            about_intro: "IT Infrastructure Engineer with a passion for cloud technologies and automation.",

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

            // About page
            about_title: "À Propos",
            about_intro: "Ingénieur Infrastructure IT passionné par les technologies cloud et l'automatisation.",

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
