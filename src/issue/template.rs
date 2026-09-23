use super::model::{RepoIssues as ListIssue, RepoIssues};
use crate::common::types;
use maud::{Markup, html};

pub fn render_issue_with_comments_mrkp(repo_name: String, issue: &ListIssue) -> Markup {
    html! {
        h2 { (repo_name) }
        hr;
        @if issue.title.is_empty() && issue.body.is_empty() {
            p { "No Comments Yet!" }
        } @else {
            h3 { (issue.title) }
            p { (issue.body) }
        }
    }
}

pub fn list_repo_issues_mrkp(repo: types::RepoIdName, issues: &[RepoIssues]) -> Markup {
    html! {
        h2 { "Issues: " (repo.name) }

        @if issues.is_empty() {
            p { "No issues on this repository yet!" }
        } @else {
            div style="overflow-x: auto;" {
                table {
                    thead {
                        tr {
                            th { "Issue Title" }
                            th { "Description" }
                            th { "Status" }
                            th { "Created" }
                            th { "Updated" }
                        }
                    }
                    tbody {
                        @for issue in issues {
                            tr {
                                td { strong { a href={ "/repo/"(repo.id)"/issue/"(issue.id) } { (issue.title) } } }
                                td { (issue.body) }
                                td {
                                    code { (issue.status) }
                                }
                                td {
                                    small { (issue.created_at) }
                                }
                                td {
                                    small { (issue.updated_at) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn new_repo_issues_mrkp(id: i64) -> Markup {
    html! {
        form method="post" action=(format!("/repo/{}/issues", id)) {
            label for="title" {
                "Issue Name"
                input type="text" id="title" name="title" required;
            }
            label for="body" {
                "Describe the issues"
                textarea id="body" name="body" rows="4" required {}
            }
            div style="max-width: 200px;" {
                button type="submit" { "Submit" }
            }
        }
    }
}
