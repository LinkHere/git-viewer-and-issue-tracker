use maud::{ html, Markup };

pub fn render_new_repo_mrkp() -> Markup {
    html! {
        h2 { "Create New Repository" }
        hr;

        form method="post" action="/new" {
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
