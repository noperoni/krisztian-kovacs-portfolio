/// Blog post data structure (compile-time static)
#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: &'static str,
    pub date: &'static str,
    pub tags: &'static [&'static str],
    pub title_en: &'static str,
    pub title_fr: &'static str,
    pub summary_en: &'static str,
    pub summary_fr: &'static str,
    pub content_en: &'static str,
    pub content_fr: &'static str,
    // New fields for blog system upgrade (HOMEL-70)
    pub reading_time_en: u32,
    pub reading_time_fr: u32,
    pub category: &'static str,
    pub featured: bool,
    pub toc_html_en: &'static str,
    pub toc_html_fr: &'static str,
    pub has_mermaid: bool,
}

impl BlogPost {
    /// Get all blog posts (sorted by date descending)
    pub fn all_posts() -> &'static [BlogPost] {
        BLOG_POSTS
    }

    /// Find a blog post by its slug
    pub fn find_by_slug(slug: &str) -> Option<&'static BlogPost> {
        BLOG_POSTS.iter().find(|p| p.slug == slug)
    }

    /// Filter posts by a specific tag
    pub fn filter_by_tag(tag: &str) -> Vec<&'static BlogPost> {
        BLOG_POSTS
            .iter()
            .filter(|p| p.tags.contains(&tag))
            .collect()
    }

    /// Get all unique tags from all posts
    pub fn all_tags() -> Vec<&'static str> {
        let mut tags: Vec<&str> = BLOG_POSTS
            .iter()
            .flat_map(|p| p.tags.iter().copied())
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }
}

// Include the generated blog posts from build.rs
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));
