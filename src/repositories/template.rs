use super::browse::{RepoContent, RepoResource};
use super::model::FetchRepos;
use chrono::{DateTime, Utc};
use maud::{Markup, PreEscaped, html};

pub fn render_repo_blob_mrkp(blob: &str) -> Markup {
    let count = blob.lines().count();
    html! {
        div.blob {
            table.blob-table {
                tr {
                    td.linenumbers {
                        pre {
                            @for n in 1..=count {
                                a id=(format!("n{n}")) href=(format!("#n{n}")) { (n) }
                                "\n"
                            }
                        }
                    }
                    td.lines {
                        pre { code { (PreEscaped(blob)) } }
                    }
                }
            }
        }
    }
}

pub fn render_repo_subtree_mrkp(repo_id: i64, repo_sub: &RepoResource) -> Markup {
    html! {
        @match repo_sub {
            RepoResource::ATree(tree_list) => {
                table {
                    th {
                        tr {
                            th {"Name"}
                            th {"Size"}
                            th {"Message"}
                            th {"Author"}
                            th {"Activity"}
                        }
                    }
                    tbody {
                        @for tree in tree_list {
                            tr {
                                td { a href={"/"(repo_id)"/tree/"(tree.repo_path)"/"(tree.repo_name)} {(tree.repo_name)}}
                                td {(tree.blob_size)}
                                td {(tree.is_dir)}
                                td {"Tippo"}
                                td {"Long Time Ago"}
                            }
                        }
                    }
                }
            }
            RepoResource::AFile {name, content, size} => {
                @let count = content.lines().count();
                small {(name) " " (size)}
                div.blob {
                    table.blob-table {
                        tr {
                            td.linenumbers {
                                pre {
                                    @for n in 1..=count {
                                        a id=(format!("n{n}")) href=(format!("#n{n}")) { (n) }
                                        "\n"
                                    }
                                }
                            }
                            td.lines {
                                pre { code { (content) } }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_repo_files_mrkp(repo_id: i64, repo_entry: RepoContent) -> Markup {
    html! {
        @match repo_entry {
            RepoContent::NoCommits => p { "No Commits Yet!" },
            RepoContent::Empty => p { "Empty Repository" },
            RepoContent::Files(entries) => {
                table {
                    thead{
                        tr {
                            th { "Name" }
                            th { "Size" }
                            th { "Message" }
                            th { "Author" }
                            th { "Activity" }
                        }
                    }
                    tbody {

                        @let mode_tree = "tree";
                        @let mode_blob = "blob";
                        @for entry in entries {
                            @if entry.is_dir {
                                tr {
                                    td { a href={ "/"(repo_id)"/"(mode_tree)"/"(entry.entry_name) } { (entry.entry_name) } }
                                    td { "---" }
                                    td { sup {(entry.commit_hash)} " " (entry.commit_message) }
                                    td { (entry.author_name) }
                                    td { (entry.last_modified) }
                                }
                            } @else if entry.is_blob {
                                tr {
                                    td { a href={ "/"(repo_id)"/"(mode_blob)"/"(entry.entry_name) } {(entry.entry_name)} }
                                    td { (entry.blob_size) }
                                    td { sup {(entry.commit_hash)} " " (entry.commit_message)}
                                    td { (entry.author_name) }
                                    td { (entry.last_modified) }
                                }
                            } @else {
                                tr {
                                    td { (entry.entry_name) }
                                    td { (entry.blob_size) }
                                    td { (entry.commit_hash)": " small {(entry.commit_message)} }
                                    td { (entry.author_name) }
                                    td { (entry.last_modified) }
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}

pub fn render_all_repos_mrkp(repo_with_dt: &[(FetchRepos, Option<DateTime<Utc>>)]) -> Markup {
    html! {
        @if repo_with_dt.is_empty() {
            p { "No repositories available yet." }
            hr;
            div {
                a href="/repo/new" role="button" { "Create New Repository" }
            }
        } @else {
            div {
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
                                        span { "—" }
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

        form method="post" action="/repo/new" {
            div {
                label for="repo_name" { "Repository Name" }
                input type="text" id="repo_name" name="repo_name" placeholder="e.g., custom-linux-kernel" required;
            }

            div {
                label for="desc" { "Repository Description (Optional)" }
                textarea id="desc" name="repo_description" rows="4" placeholder="Brief details about this repository..." {}
            }

            button type="submit" { "Create" }
        }
    }
}
