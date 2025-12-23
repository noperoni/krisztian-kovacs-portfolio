use leptos::prelude::*;
use leptos_meta::Script;
use leptos_router::hooks::use_query_map;

use crate::blog::BlogPost;
use crate::i18n::{use_i18n, I18nContext};

// ============================================================================
// CONSTANTS
// ============================================================================

const POSTS_PER_PAGE: usize = 6;

// ============================================================================
// BLOG LIST PAGE
// ============================================================================

/// Blog listing page with tag filtering and pagination
#[component]
pub fn BlogPage() -> impl IntoView {
    let i18n = use_i18n();

    // Read ?tag=X from URL query string
    let query = use_query_map();
    let initial_tag = query.read().get("tag");

    // Tag filter state (initialized from URL if present)
    let selected_tag = RwSignal::new(initial_tag);

    // Sync selected_tag with URL changes (for back/forward navigation)
    Effect::new(move |_| {
        let url_tag = query.read().get("tag");
        if url_tag != selected_tag.get() {
            selected_tag.set(url_tag);
        }
    });

    // Pagination state
    let current_page = RwSignal::new(1usize);

    // Reset to page 1 when tag changes
    Effect::new(move |_| {
        let _ = selected_tag.get();
        current_page.set(1);
    });

    // Filtered posts based on selected tag
    let filtered_posts = Signal::derive(move || {
        let all = BlogPost::all_posts();
        match selected_tag.get() {
            Some(tag) => all.iter().filter(|p| p.tags.contains(&tag.as_str())).collect::<Vec<_>>(),
            None => all.iter().collect::<Vec<_>>(),
        }
    });

    // Total pages calculation
    let total_pages = Signal::derive(move || {
        let count = filtered_posts.get().len();
        (count + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE.max(1)
    });

    // Paginated posts for current page
    let paginated_posts = Signal::derive(move || {
        let all = filtered_posts.get();
        let page = current_page.get();
        let start = (page - 1) * POSTS_PER_PAGE;
        all.into_iter().skip(start).take(POSTS_PER_PAGE).collect::<Vec<_>>()
    });

    // Get all unique tags
    let all_tags = BlogPost::all_tags();

    view! {
        <div class="blog-page">
            <BlogHeader i18n=i18n />
            <div class="blog-container">
                <TagFilter tags=all_tags selected=selected_tag i18n=i18n />
                <BlogGrid posts=paginated_posts i18n=i18n />
                <Pagination
                    current=current_page
                    total=total_pages
                    i18n=i18n
                />
            </div>
        </div>
    }
}

// ============================================================================
// BLOG HEADER
// ============================================================================

#[component]
fn BlogHeader(i18n: I18nContext) -> impl IntoView {
    view! {
        <header class="page-header">
            <h1 class="page-title">{move || i18n.t().blog_page_title}</h1>
            <p class="page-subtitle">{move || i18n.t().blog_page_subtitle}</p>
        </header>
    }
}

// ============================================================================
// TAG FILTER
// ============================================================================

#[component]
fn TagFilter(
    tags: Vec<&'static str>,
    selected: RwSignal<Option<String>>,
    i18n: I18nContext,
) -> impl IntoView {
    view! {
        <div class="filter-tabs">
            <button
                class=move || if selected.get().is_none() { "filter-tab active" } else { "filter-tab" }
                on:click=move |_| selected.set(None)
            >
                {move || i18n.t().blog_filter_all}
            </button>
            {tags.into_iter().map(|tag| {
                let tag_owned = tag.to_string();
                let tag_for_check = tag.to_string();
                let tag_display = tag.to_string();
                view! {
                    <button
                        class=move || {
                            if selected.get().as_deref() == Some(&tag_for_check) {
                                "filter-tab active"
                            } else {
                                "filter-tab"
                            }
                        }
                        on:click=move |_| selected.set(Some(tag_owned.clone()))
                    >
                        {tag_display}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

// ============================================================================
// BLOG GRID
// ============================================================================

#[component]
fn BlogGrid(posts: Signal<Vec<&'static BlogPost>>, i18n: I18nContext) -> impl IntoView {
    view! {
        <div class="blog-grid">
            <For
                each=move || posts.get().into_iter().enumerate()
                key=|(_, post)| post.slug
                children=move |(index, post)| {
                    view! {
                        <BlogCard post=post i18n=i18n index=index />
                    }
                }
            />
        </div>
    }
}

// ============================================================================
// BLOG CARD
// ============================================================================

#[component]
fn BlogCard(post: &'static BlogPost, i18n: I18nContext, index: usize) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);
    let animation_delay = format!("animation-delay: {}s", index as f32 * 0.1);

    view! {
        <article class="blog-card" style=animation_delay>
            <a href=href class="blog-card-link">
                <div class="blog-card-header">
                    <time class="blog-date">{post.date}</time>
                    <div class="blog-tags">
                        {post.tags.iter().map(|tag| view! {
                            <span class="tag">{*tag}</span>
                        }).collect_view()}
                    </div>
                </div>
                <h2 class="blog-title">
                    {move || if i18n.is_french() { post.title_fr } else { post.title_en }}
                </h2>
                <p class="blog-summary">
                    {move || if i18n.is_french() { post.summary_fr } else { post.summary_en }}
                </p>
                <span class="read-more">
                    {move || i18n.t().blog_read_more}
                    " →"
                </span>
            </a>
        </article>
    }
}

// ============================================================================
// PAGINATION
// ============================================================================

#[component]
fn Pagination(
    current: RwSignal<usize>,
    total: Signal<usize>,
    i18n: I18nContext,
) -> impl IntoView {
    view! {
        <Show when=move || { total.get() > 1 }>
            <nav class="pagination" aria-label="Blog pagination">
                <button
                    class="pagination-btn"
                    disabled=move || current.get() <= 1
                    on:click=move |_| current.update(|p| *p = (*p).saturating_sub(1).max(1))
                >
                    "← " {move || i18n.t().blog_prev}
                </button>
                <span class="pagination-info">
                    {move || current.get()} " / " {move || total.get()}
                </span>
                <button
                    class="pagination-btn"
                    disabled=move || current.get() >= total.get()
                    on:click=move |_| current.update(|p| *p = (*p + 1).min(total.get()))
                >
                    {move || i18n.t().blog_next} " →"
                </button>
            </nav>
        </Show>
    }
}

// ============================================================================
// BLOG TAG PAGE (Route: /blog/tags/:tag)
// ============================================================================

/// Blog page filtered by tag from URL path
#[component]
pub fn BlogTagPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = leptos_router::hooks::use_params_map();

    // Get tag from URL path param
    let tag_from_path = Signal::derive(move || params.read().get("tag"));

    // Tag filter state (initialized from path param)
    let selected_tag = RwSignal::new(tag_from_path.get());

    // Sync with path param changes
    Effect::new(move |_| {
        let path_tag = tag_from_path.get();
        if path_tag != selected_tag.get() {
            selected_tag.set(path_tag);
        }
    });

    // Pagination state
    let current_page = RwSignal::new(1usize);

    // Reset to page 1 when tag changes
    Effect::new(move |_| {
        let _ = selected_tag.get();
        current_page.set(1);
    });

    // Filtered posts based on selected tag
    let filtered_posts = Signal::derive(move || {
        let all = BlogPost::all_posts();
        match selected_tag.get() {
            Some(tag) => all.iter().filter(|p| p.tags.contains(&tag.as_str())).collect::<Vec<_>>(),
            None => all.iter().collect::<Vec<_>>(),
        }
    });

    // Total pages calculation
    let total_pages = Signal::derive(move || {
        let count = filtered_posts.get().len();
        (count + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE.max(1)
    });

    // Paginated posts for current page
    let paginated_posts = Signal::derive(move || {
        let all = filtered_posts.get();
        let page = current_page.get();
        let start = (page - 1) * POSTS_PER_PAGE;
        all.into_iter().skip(start).take(POSTS_PER_PAGE).collect::<Vec<_>>()
    });

    // Get all unique tags
    let all_tags = BlogPost::all_tags();

    view! {
        <div class="blog-page">
            <BlogHeader i18n=i18n />
            <div class="blog-container">
                <TagFilterLinks tags=all_tags current_tag=selected_tag i18n=i18n />
                <BlogGrid posts=paginated_posts i18n=i18n />
                <Pagination
                    current=current_page
                    total=total_pages
                    i18n=i18n
                />
            </div>
        </div>
    }
}

// ============================================================================
// TAG FILTER WITH LINKS (for /blog/tags/:tag routes)
// ============================================================================

#[component]
fn TagFilterLinks(
    tags: Vec<&'static str>,
    current_tag: RwSignal<Option<String>>,
    i18n: I18nContext,
) -> impl IntoView {
    view! {
        <div class="filter-tabs">
            <a
                href="/blog"
                class=move || if current_tag.get().is_none() { "filter-tab active" } else { "filter-tab" }
            >
                {move || i18n.t().blog_filter_all}
            </a>
            {tags.into_iter().map(|tag| {
                let tag_for_check = tag.to_string();
                let tag_display = tag.to_string();
                let href = format!("/blog/tags/{}", tag);
                view! {
                    <a
                        href=href
                        class=move || {
                            if current_tag.get().as_deref() == Some(&tag_for_check) {
                                "filter-tab active"
                            } else {
                                "filter-tab"
                            }
                        }
                    >
                        {tag_display}
                    </a>
                }
            }).collect_view()}
        </div>
    }
}

// ============================================================================
// BLOG POST DETAIL PAGE
// ============================================================================

/// Individual blog post page with ToC, JSON-LD SEO, and Mermaid support
#[component]
pub fn BlogPostPage() -> impl IntoView {
    let i18n = use_i18n();

    // Get slug from URL params
    let params = leptos_router::hooks::use_params_map();

    // Find post by slug
    let post = Signal::derive(move || {
        let slug = params.get().get("slug").unwrap_or_default();
        BlogPost::find_by_slug(&slug)
    });

    view! {
        <div class="blog-post-page">
            {move || match post.get() {
                Some(post) => {
                    // Generate JSON-LD schema for SEO
                    let json_ld = generate_json_ld(post, i18n.is_french());

                    view! {
                        // JSON-LD structured data for rich search results
                        <Script type_="application/ld+json">{json_ld}</Script>

                        // Conditional Mermaid loader (only if post has diagrams)
                        {post.has_mermaid.then(|| view! {
                            <Script type_="module">
                                r#"import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
                                mermaid.initialize({ startOnLoad: true, theme: 'default' });"#
                            </Script>
                        })}

                        // Header outside the layout (full width)
                        <header class="post-header">
                            <a href="/blog" class="back-link">
                                "← " {move || i18n.t().blog_back_to_list}
                            </a>
                            <h1 class="post-title">
                                {move || if i18n.is_french() { post.title_fr } else { post.title_en }}
                            </h1>
                            <div class="post-meta">
                                <time class="post-date">{post.date}</time>
                                <span class="reading-time">
                                    "📖 "
                                    {move || if i18n.is_french() { post.reading_time_fr } else { post.reading_time_en }}
                                    " min"
                                </span>
                                <div class="post-tags">
                                    {post.tags.iter().map(|tag| {
                                        let tag_str = *tag;
                                        view! {
                                            <a href=format!("/blog/tags/{}", tag_str) class="tag">{tag_str}</a>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </header>

                        // Layout container for ToC sidebar + content
                        <div class="blog-post-layout">
                            // Table of Contents (sidebar on wide screens)
                            {move || {
                                let toc = if i18n.is_french() { post.toc_html_fr } else { post.toc_html_en };
                                (!toc.is_empty()).then(|| view! {
                                    <aside class="blog-toc-wrapper">
                                        <nav class="blog-toc" aria-label="Table of contents">
                                            <h2 class="toc-title">
                                                {move || if i18n.is_french() { "Table des matières" } else { "Table of Contents" }}
                                            </h2>
                                            <div class="toc-content" inner_html=toc />
                                        </nav>
                                    </aside>
                                })
                            }}

                            <article class="blog-post">
                                <div
                                    class="post-content prose"
                                    inner_html=move || {
                                        if i18n.is_french() { post.content_fr } else { post.content_en }
                                    }
                                />
                                <footer class="post-footer">
                                    <a href="/blog" class="btn btn-secondary">
                                        "← " {move || i18n.t().blog_back_to_list}
                                    </a>
                                </footer>
                            </article>
                        </div>
                    }.into_any()
                },
                None => view! {
                    <div class="not-found">
                        <h1>{move || i18n.t().blog_not_found}</h1>
                        <p>{move || i18n.t().blog_not_found_desc}</p>
                        <a href="/blog" class="btn btn-primary">
                            {move || i18n.t().blog_back_to_list}
                        </a>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}

/// Generate JSON-LD BlogPosting schema for SEO rich results
fn generate_json_ld(post: &'static BlogPost, is_french: bool) -> String {
    let title = if is_french { post.title_fr } else { post.title_en };
    let summary = if is_french { post.summary_fr } else { post.summary_en };
    let tags = post.tags.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ");

    format!(
        r#"{{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "{}",
  "description": "{}",
  "author": {{
    "@type": "Person",
    "name": "Kovács Krisztián Géza"
  }},
  "datePublished": "{}",
  "keywords": [{}],
  "mainEntityOfPage": {{
    "@type": "WebPage",
    "@id": "https://kovacs.pilgrim.ovh/blog/{}"
  }}
}}"#,
        title.replace('"', "\\\""),
        summary.replace('"', "\\\""),
        post.date,
        tags,
        post.slug
    )
}
