use crate::html::preprocessor::generic_preprocessor;
use maud::{Markup, PreEscaped, Render, html};
use pulldown_cmark::Parser;
use regex::Captures;
use rocket::Route;
use rocket::tokio::io::AsyncReadExt;
use rocket::{get, routes};
use std::path::{Path, PathBuf};

pub fn get_routes() -> Vec<Route> {
    routes![blog_post, embedable_post]
}

#[get("/embedable/<post>")]
async fn embedable_post(post: PathBuf) -> Option<Markup> {
    let md_path = Path::new("webserver/static/blog")
        .join(post)
        .with_extension("md");

    // Read and return raw markdown content
    match rocket::fs::NamedFile::open(md_path).await {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).await.ok();
            let markdown = Markdown(buffer);

            Some(embadable_blog_page(markdown))
        }
        Err(_) => None,
    }
}

#[get("/<post>")]
async fn blog_post(post: PathBuf) -> Option<Markup> {
    let md_path = Path::new("webserver/static/blog")
        .join(post)
        .with_extension("md");

    // Read and return raw markdown content
    match rocket::fs::NamedFile::open(md_path).await {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).await.ok();
            let markdown = Markdown(buffer);

            Some(blog_page(markdown))
        }
        Err(_) => None,
    }
}

// TODO: refactor to avoid code duplication over these two pages
fn embadable_blog_page(md: Markdown<String>) -> Markup {
    html! {
        head {
            link rel = "stylesheet" href = "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.colors.min.css"{}
            style { "
                .code-display {
                        border-radius: 8px;
                        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                        margin: 1.5em 0;
                        overflow: hidden;
                    }

                    .code-header {
                        //background-color: 2d2d2d;
                        padding: 8px 16px;
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        border-bottom: 1px solid #404040;
                    }

                    .language-label {
                        color: #cccccc;
                        font-family: 'SF Mono', Menlo, monospace;
                        font-size: 0.9em;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    }

                    .copy-button {
                        background: #3d3d3d;
                        border: 1px solid #5a5a5a;
                        color: #a0a0a0;
                        padding: 4px 12px;
                        border-radius: 4px;
                        cursor: pointer;
                        font-family: system-ui, sans-serif;
                        font-size: 0.8em;
                        transition: all 0.2s ease;
                    }

                    .copy-button:hover {
                        background: #4d4d4d;
                        color: #ffffff;
                    }

                    .code-container {
                        margin: 0;
                        padding: 16px;
                        //background-color: #1e1e1e;
                        white-space: pre-wrap; /* Changed from default 'pre' */
                        word-wrap: break-word;
                        overflow-wrap: break-word;
                    }

                    .code-container code {
                        font-family: 'SF Mono', Menlo, monospace;
                        font-size: 0.95em;
                        line-height: 1.5;
                        color: #d4d4d4;
                        display: block;
                        white-space: pre-wrap; /* Changed from default 'pre' */
                    }
            " }
        }
        (md.render())
    }
}

fn blog_page(md: Markdown<String>) -> Markup {
    generic_preprocessor(html! {
        head {
            link rel = "stylesheet" href = "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.colors.min.css"{}
            style { "
                .code-display {
                        border-radius: 8px;
                        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                        margin: 1.5em 0;
                        overflow: hidden;
                    }

                    .code-header {
                        //background-color: 2d2d2d;
                        padding: 8px 16px;
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        border-bottom: 1px solid #404040;
                    }

                    .language-label {
                        color: #cccccc;
                        font-family: 'SF Mono', Menlo, monospace;
                        font-size: 0.9em;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    }

                    .copy-button {
                        background: #3d3d3d;
                        border: 1px solid #5a5a5a;
                        color: #a0a0a0;
                        padding: 4px 12px;
                        border-radius: 4px;
                        cursor: pointer;
                        font-family: system-ui, sans-serif;
                        font-size: 0.8em;
                        transition: all 0.2s ease;
                    }

                    .copy-button:hover {
                        background: #4d4d4d;
                        color: #ffffff;
                    }

                    .code-container {
                        margin: 0;
                        padding: 16px;
                        //background-color: #1e1e1e;
                        white-space: pre-wrap; /* Changed from default 'pre' */
                        word-wrap: break-word;
                        overflow-wrap: break-word;
                    }

                    .code-container code {
                        font-family: 'SF Mono', Menlo, monospace;
                        font-size: 0.95em;
                        line-height: 1.5;
                        color: #d4d4d4;
                        display: block;
                        white-space: pre-wrap; /* Changed from default 'pre' */
                    }
            " }
        }
        main class="container"{
            (md.render())
        }
    })
}

fn html_codeblock_postprocessor(html: String) -> String {
    // postprocessing html... am i in hell?
    let re =
        regex::Regex::new(r#"(?s)<pre><code class="language-(.*?)">(.*?)</code></pre>"#).unwrap(); // postprocessing it with regex? yeah this really is hell
    re.replace_all(html.as_str(), |captures: &Captures| {
        let lang = &captures[1];
        let code = &captures[2];
        format!("{}", custom_code_block(code, lang).into_string())
    })
    .into_owned()
}

fn custom_code_block(code: &str, lang: &str) -> Markup {
    let lang_formatted = format!("language-{}", lang);
    html! {
        div class="code-display"{
            div class = "code-header pico-background-sand-700" {
                span class="language-label" { (lang) }
                button class="copy-button" { "Copy" }
            }
            pre class = "code-container pico-background-sand-850"{
                code class=(lang_formatted) {(code)}
            }
        }
    }
}

/// Renders a block of Markdown using `pulldown-cmark`.
pub struct Markdown<T>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        let mut unsafe_html = String::new();
        let parser = Parser::new(self.0.as_ref());
        pulldown_cmark::html::push_html(&mut unsafe_html, parser);
        // Sanitize it with ammonia
        let unsafe_html = html_codeblock_postprocessor(unsafe_html);
        // let safe_html = ammonia::clean(&unsafe_html); who needs checked html anyways right?
        PreEscaped(unsafe_html)
    }
}
