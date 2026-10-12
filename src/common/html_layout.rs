use maud::{DOCTYPE, Markup, html};

pub fn layout(
    page_title: &str,
    repo_header: (String, Option<String>),
    page_content: Markup,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/static/sakura.css";
                link rel="stylesheet" href="/static/style.css";
                title { (page_title) " | Issue Tracker" }
            }
            body {
                header class="site-header" {
                    div class="info" {
                        div class="row" {
                            h4 { (repo_header.0) }
                            select class="menu" aria-label="Menu" {
                                option { "main" }
                                option { "dev" }
                            }
                        }
                        hr;
                        div class="row" {
                            span class="grow" { @if let Some(desc) = repo_header.1 { (desc) } @else { "No Description" } }
                            span class="author" { "John Doe" }
                        }
                    }
                    img src="/static/game.png" alt="icon" width="128" height="128";
                }

                hr;

                main {
                    (page_content)
                }

                hr;

                footer {
                    small { "© 2026 Issue Tracker. All rights reserved." }
                }
            }
        }
    }
}
