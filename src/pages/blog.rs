use leptos::prelude::*;

use crate::blog::BlogPost;
use crate::i18n::{use_i18n, I18nContext};

// ============================================================================
// BLOG LIST PAGE
// ============================================================================

/// Blog listing page with tag filtering
#[component]
pub fn BlogPage() -> impl IntoView {
    let i18n = use_i18n();

    // Tag filter state
    let selected_tag = RwSignal::new(Option::<String>::None);

    // Filtered posts based on selected tag
    let filtered_posts = Signal::derive(move || {
        let all = BlogPost::all_posts();
        match selected_tag.get() {
            Some(tag) => all.iter().filter(|p| p.tags.contains(&tag.as_str())).collect(),
            None => all.iter().collect(),
        }
    });

    // Get all unique tags
    let all_tags = BlogPost::all_tags();

    view! {
        <div class="blog-page">
            <BlogHeader i18n=i18n.clone() />
            <div class="blog-container">
                <TagFilter tags=all_tags selected=selected_tag i18n=i18n.clone() />
                <BlogGrid posts=filtered_posts i18n=i18n />
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
                        <BlogCard post=post i18n=i18n.clone() index=index />
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
// BLOG POST DETAIL PAGE
// ============================================================================

/// Individual blog post page
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
                Some(post) => view! {
                    <article class="blog-post">
                        <header class="post-header">
                            <a href="/blog" class="back-link">
                                "← " {move || i18n.t().blog_back_to_list}
                            </a>
                            <h1 class="post-title">
                                {move || if i18n.is_french() { post.title_fr } else { post.title_en }}
                            </h1>
                            <div class="post-meta">
                                <time class="post-date">{post.date}</time>
                                <div class="post-tags">
                                    {post.tags.iter().map(|tag| {
                                        let tag_str = *tag;
                                        view! {
                                            <a href=format!("/blog?tag={}", tag_str) class="tag">{tag_str}</a>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </header>
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
                }.into_any(),
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
