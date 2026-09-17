use super::model::Repo;
use maud::{Markup, html};

pub fn list_repo_mrkp(repos: &[Repo]) -> Markup {
    html! {
        @if repos.is_empty() {
            p { "No repositories available yet." }
        } @else {
            div style="overflow-x: auto;" {
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Description" }
                            th { "URL" }
                            th { "Created" }
                        }
                    }
                    tbody {
                        @for repo in repos {
                            tr {
                                td {
                                    strong {
                                        a href={ "/repo/" (repo.id) } { (repo.name) }
                                    }
                                }
                                td {
                                    @if let Some(desc) = &repo.description {
                                        (desc)
                                    } @else {
                                        span style="opacity: 0.5;" { "—" }
                                    }
                                }
                                td {
                                    code { (repo.cgit_url) }
                                }
                                td {
                                    small { (repo.created_at) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn new_repo_mrkp() -> Markup {
    html! {
        h2 { "Create New Repository" }
        hr;

        form method="post" action="/repos/new" {

            label for="name" {
                "Repository Name"
                input type="text" id="name" name="name" placeholder="e.g., custom-linux-kernel" required;
            }

            label for="repo_url" {
                "Repository URL"
                input type="text" id="repo_url" name="cgit_url" placeholder="https://example.com" required;
            }

            label for="desc" {
                "Repository Description (Optional)"
                textarea id="desc" name="description" rows="4" placeholder="Brief details about this tree..." {}
            }

            div style="max-width: 200px;" {
                button type="submit" { "Create Repository" }
            }
        }
    }
}
