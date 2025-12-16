use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct BlogFrontmatter {
    slug: String,
    date: String,
    tags: Vec<String>,
    title_en: String,
    title_fr: String,
    summary_en: String,
    summary_fr: String,
}

#[derive(Debug)]
struct ParsedBlogPost {
    slug: String,
    date: String,
    tags: Vec<String>,
    title_en: String,
    title_fr: String,
    summary_en: String,
    summary_fr: String,
    content_en: String,
    content_fr: String,
}

fn main() {
    println!("cargo:rerun-if-changed=content/blog");

    let blog_dir = Path::new("content/blog");

    // If no blog directory exists, generate empty posts array
    if !blog_dir.exists() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let generated = generate_empty_rust_code();
        fs::write(Path::new(&out_dir).join("blog_posts.rs"), generated).unwrap();
        return;
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let mut posts = Vec::new();

    for entry in WalkDir::new(blog_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());

        let content = fs::read_to_string(entry.path()).unwrap_or_else(|e| {
            panic!("Failed to read {}: {}", entry.path().display(), e);
        });

        match parse_blog_post(&content, &ss) {
            Ok(post) => posts.push(post),
            Err(e) => {
                eprintln!(
                    "cargo:warning=Failed to parse {}: {}",
                    entry.path().display(),
                    e
                );
            }
        }
    }

    // Sort by date descending (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    // Generate Rust code
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let generated = generate_rust_code(&posts);
    fs::write(Path::new(&out_dir).join("blog_posts.rs"), generated).unwrap();

    println!(
        "cargo:warning=Blog: Generated {} posts to OUT_DIR/blog_posts.rs",
        posts.len()
    );
}

fn parse_blog_post(content: &str, ss: &SyntaxSet) -> Result<ParsedBlogPost, String> {
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(content);

    let frontmatter: BlogFrontmatter = parsed
        .data
        .ok_or("No frontmatter found")?
        .deserialize()
        .map_err(|e| format!("Invalid frontmatter: {}", e))?;

    // Split content by language markers
    let body = parsed.content;
    let (content_en, content_fr) = split_bilingual_content(&body);

    // Convert markdown to HTML with syntax highlighting
    let html_en = markdown_to_html(&content_en, ss);
    let html_fr = markdown_to_html(&content_fr, ss);

    Ok(ParsedBlogPost {
        slug: frontmatter.slug,
        date: frontmatter.date,
        tags: frontmatter.tags,
        title_en: frontmatter.title_en,
        title_fr: frontmatter.title_fr,
        summary_en: frontmatter.summary_en,
        summary_fr: frontmatter.summary_fr,
        content_en: html_en,
        content_fr: html_fr,
    })
}

fn split_bilingual_content(content: &str) -> (String, String) {
    // Split on <!-- FR --> marker
    let parts: Vec<&str> = content.split("<!-- FR -->").collect();

    let en_part = parts
        .first()
        .map(|s| s.replace("<!-- EN -->", "").trim().to_string())
        .unwrap_or_default();

    let fr_part = parts
        .get(1)
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    (en_part, fr_part)
}

fn markdown_to_html(markdown: &str, ss: &SyntaxSet) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);
    let events: Vec<Event> = parser.collect();

    let mut output = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                in_code_block = true;
                code_lang = lang.to_string();
                code_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let highlighted = highlight_code(&code_content, &code_lang, ss);
                output.push_str(&highlighted);
                code_lang.clear();
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(&text);
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                in_code_block = true;
                code_lang = "text".to_string();
                code_content.clear();
            }
            // Handle inline elements
            Event::Start(Tag::Paragraph) => output.push_str("<p>"),
            Event::End(TagEnd::Paragraph) => output.push_str("</p>\n"),
            Event::Start(Tag::Heading { level, .. }) => {
                output.push_str(&format!("<h{}>", level as u8))
            }
            Event::End(TagEnd::Heading(level)) => {
                output.push_str(&format!("</h{}>\n", level as u8))
            }
            Event::Start(Tag::Strong) => output.push_str("<strong>"),
            Event::End(TagEnd::Strong) => output.push_str("</strong>"),
            Event::Start(Tag::Emphasis) => output.push_str("<em>"),
            Event::End(TagEnd::Emphasis) => output.push_str("</em>"),
            Event::Start(Tag::Link { dest_url, .. }) => {
                output.push_str(&format!("<a href=\"{}\">", escape_html(&dest_url)))
            }
            Event::End(TagEnd::Link) => output.push_str("</a>"),
            Event::Start(Tag::List(None)) => output.push_str("<ul>\n"),
            Event::End(TagEnd::List(false)) => output.push_str("</ul>\n"),
            Event::Start(Tag::List(Some(start))) => {
                output.push_str(&format!("<ol start=\"{}\">\n", start))
            }
            Event::End(TagEnd::List(true)) => output.push_str("</ol>\n"),
            Event::Start(Tag::Item) => output.push_str("<li>"),
            Event::End(TagEnd::Item) => output.push_str("</li>\n"),
            Event::Start(Tag::BlockQuote) => output.push_str("<blockquote>"),
            Event::End(TagEnd::BlockQuote) => output.push_str("</blockquote>\n"),
            Event::Code(code) => {
                output.push_str(&format!("<code>{}</code>", escape_html(&code)))
            }
            Event::Text(text) if !in_code_block => {
                output.push_str(&escape_html(&text));
            }
            Event::SoftBreak => output.push('\n'),
            Event::HardBreak => output.push_str("<br>\n"),
            Event::Rule => output.push_str("<hr>\n"),
            _ => {}
        }
    }

    output
}

fn highlight_code(code: &str, lang: &str, ss: &SyntaxSet) -> String {
    // Find syntax or fall back to plain text
    let syntax = ss
        .find_syntax_by_token(lang)
        .or_else(|| ss.find_syntax_by_extension(lang))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, ss, syntect::html::ClassStyle::Spaced);

    for line in syntect::util::LinesWithEndings::from(code) {
        let _ = generator.parse_html_for_line_which_includes_newline(line);
    }

    let highlighted = generator.finalize();

    format!(
        r#"<pre class="code-block" data-lang="{}"><code class="language-{}">{}</code></pre>"#,
        escape_html(lang),
        escape_html(lang),
        highlighted
    )
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn generate_rust_code(posts: &[ParsedBlogPost]) -> String {
    let mut code = String::from(
        r#"// Auto-generated by build.rs - DO NOT EDIT

pub static BLOG_POSTS: &[crate::blog::data::BlogPost] = &[
"#,
    );

    for post in posts {
        // Escape strings for Rust raw literals
        let tags_str = post
            .tags
            .iter()
            .map(|t| format!("\"{}\"", t))
            .collect::<Vec<_>>()
            .join(", ");

        code.push_str(&format!(
            r####"    crate::blog::data::BlogPost {{
        slug: "{}",
        date: "{}",
        tags: &[{}],
        title_en: r#"{}"#,
        title_fr: r#"{}"#,
        summary_en: r#"{}"#,
        summary_fr: r#"{}"#,
        content_en: r###"{}"###,
        content_fr: r###"{}"###,
    }},
"####,
            post.slug,
            post.date,
            tags_str,
            post.title_en,
            post.title_fr,
            post.summary_en,
            post.summary_fr,
            post.content_en,
            post.content_fr,
        ));
    }

    code.push_str("];\n");
    code
}

fn generate_empty_rust_code() -> String {
    r#"// Auto-generated by build.rs - DO NOT EDIT

pub static BLOG_POSTS: &[crate::blog::data::BlogPost] = &[];
"#
    .to_string()
}
