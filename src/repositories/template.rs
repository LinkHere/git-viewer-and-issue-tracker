use super::model::FetchRepos;
use maud::{ html, Markup };

pub fn render_all_repos_mrkp(
    repos: &[FetchRepos],
    timestamp: &str
) -> Markup {
	html! {
        @if repos.is_empty() {
            p { "No repositories available yet." }
            hr;
            div style="max-width: 200px; margin-top: 2px;"{
                a href="/repo/new" role="button" { "Create New Repository" }
            }
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
                                    small { (repo.created_at)"|"(timestamp) }
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

        form.form-container method="post" action="/repo/new" {
            div.form-group {
                label.form-label for="repo_name" { "Repository Name" }
                input.form-control type="text" id="repo_name" name="repo_name" placeholder="e.g., custom-linux-kernel" required;
            }
            
            div.form-group.align-start {
                label.form-label for="desc" { "Repository Description (Optional)" }
                textarea.form-control id="desc" name="repo_description" rows="4" placeholder="Brief details about this repository..." {}
            }
            
            button.btn-primary.align-right type="submit" { "Create" }
        }
    }
}
