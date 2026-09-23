use super::model::FetchRepos;
use maud::{ html, Markup };

pub fn render_all_repos_mrkp(repos: &[FetchRepos]) -> Markup {
	html! {
        @if repos.is_empty() {
            p { "No repositories available yet." }
        } @else {
            div style="max-width: 200px;"{
                a href="/repo/new" role="button" { "Create New Repository" }
            }
            div style="overflow-x: auto;" {
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Description" }
                            th { "Created" }
                        }
                    }
                    tbody {
                        @for repo in repos {
                            tr {
                                td {
                                    strong {
                                        a href={ "/repo/" (repo.id) } { (repo.repo_name) }
                                    }
                                }
                                td {
                                    @if let Some(desc) = &repo.repo_description {
                                        (desc)
                                    } @else {
                                        span style="opacity: 0.5;" { "—" }
                                    }
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

pub fn render_new_repo_mrkp() -> Markup {
    html! {
        h2 { "Create New Repository" }
        hr;

        form method="post" action="/repo/new" {
            label for="repo_name" {
                "Repository Name"
                input type="text" id="repo_name" name="repo_name" placeholder="e.g., custom-linux-kernel" required;
            }
            label for="desc" {
                "Repository Description (Optional)"
                textarea id="desc" name="repo_description" rows="4" placeholder="Brief details about this repository..." {}
            }
            div style="max-width: 200px;" {
                button type="submit" { "Create" }
            }
        }
    }
}
