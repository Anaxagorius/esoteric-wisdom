use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use pulldown_cmark::{Options, Parser, html};

use crate::auth::HtmlTemplate;

#[derive(Template)]
#[template(path = "content_page.html")]
struct ContentPageTemplate {
    page_title: String,
    meta_description: String,
    heading: String,
    lead: String,
    body_html: String,
}

pub async fn render_markdown_page(slug: &str) -> impl IntoResponse {
    let root = std::env::var("CONTENT_PAGES_DIR").unwrap_or_else(|_| "content/pages".to_string());
    let file_path = format!("{root}/{slug}.md");

    let content = match tokio::fs::read_to_string(&file_path).await {
        Ok(content) => content,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let (title, description, markdown_body) = parse_markdown_metadata(&content);
    let body_html = markdown_to_html(&markdown_body);
    HtmlTemplate(ContentPageTemplate {
        page_title: format!("{title} — Esoteric Wisdom"),
        meta_description: description.clone(),
        heading: title,
        lead: description,
        body_html,
    })
    .into_response()
}

fn parse_markdown_metadata(content: &str) -> (String, String, String) {
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut body_lines = Vec::new();

    let mut in_header = true;
    for line in content.lines() {
        if in_header {
            if line.trim().is_empty() {
                in_header = false;
                continue;
            }
            if let Some(value) = line.strip_prefix("title:") {
                title = Some(value.trim().to_string());
                continue;
            }
            if let Some(value) = line.strip_prefix("description:") {
                description = Some(value.trim().to_string());
                continue;
            }
            in_header = false;
        }
        body_lines.push(line);
    }

    let body = body_lines.join("\n");
    let heading_from_md = body
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(|v| v.trim().to_string()))
        .unwrap_or_else(|| "Esoteric Wisdom".to_string());
    let title = title.unwrap_or_else(|| heading_from_md.clone());
    let description = description.unwrap_or_else(|| "Esoteric Wisdom reference page.".to_string());
    (title, description, body)
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
