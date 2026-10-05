use super::browse::RepoContent;
use super::model::FetchRepos;
use chrono::{ DateTime,Utc };
use maud::{ html, Markup };

pub fn render_repo_files_mrkp(repo_entry: RepoContent) -> Markup {
    html! {
        @match repo_entry {
            RepoContent::NoCommits => p { "No Commits Yet!" },
            RepoContent::Empty => p { "Empty Repository" },
            RepoContent::Files(entries) => {
                ul {
                    @for entry in entries {
                        @if entry.is_dir {
                            li { a href={ "/tree/"(entry.entry_name) } {(entry.entry_name)} "----" small { (entry.is_dir) } }
                        } @else {
                            li { a href={ "/blob/"(entry.entry_name) } {(entry.entry_name)} "----" small { (entry.is_dir) } }
                        }
                    }
                }
            },
        }        
    }
}

pub fn render_all_repos_mrkp(
    repo_with_dt: &[(FetchRepos, Option<DateTime<Utc>>)],
) -> Markup {
	html! {
        @if repo_with_dt.is_empty() {
            p { "No repositories available yet." }
            hr;
            div style="max-width: 200px; margin-top: 2px;"{
                a href="/repo/new" role="button" { "Create New Repository" }
            }
        } @else {
            div style="max-width: 200px;"{
                a href="/repo/new" role="button" { "Create New Repository" }
            }
            div {
                table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Description" }
                            th { "Created" }
                            th { "Last Activity" }
                        }
                    }
                    tbody {
                        @for (repo, dt) in repo_with_dt {
                            tr {
                                td {
                                    strong {
                                        a href={ "/" (repo.id) } { (repo.repo_name) }
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
                                td {
                                    @if let Some(last_dt_updated) = dt {
                                        small { (last_dt_updated) }
                                    } @else {
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
