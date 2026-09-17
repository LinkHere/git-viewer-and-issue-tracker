use maud::{DOCTYPE, Markup, html};

pub fn layout(page_title: &str, page_content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light dark";
                link rel="stylesheet" href="/static/pico.fluid.classless.css";
                title { (page_title) " | Issue Tracker" }
            }
            body {
                header {
                    nav {
                        ul {
                            li { strong { "Issue Tracker" } }
                        }
                        ul {
                            li { a href="/" { "Home" } }
                            li { a href="/about" { "About" } }
                            li { a href="/contact" { "Contact" } }
                        }
                    }
                }

                main {
                    (page_content)
                }

                footer {
                    small { "© 2026 Issue Tracker. All rights reserved." }
                }
            }
        }
    }
}
