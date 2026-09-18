use super::model::{RepoIssues as ListIssue, RepoIssues};
use maud::{Markup, html};

pub fn render_issue_with_comments_mrkp(repo_name: String, issue: &ListIssue) -> Markup {
	html! {
		@if repo_name.is_empty() {
			h2 { (repo_name) }
			hr;
			p { "No Comments Yet!" }
		} @else {
			h2 { (repo_name) }
			hr;
			h3 { (issue.title) }
			p { (issue.body) }
		}
	}
}

pub fn list_repo_issues_mrkp(repo_name: &str, issues: &[RepoIssues]) -> Markup {
    html! {
        h2 { "Issues: " (repo_name) }

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
                                td { strong { (issue.title) } }
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
