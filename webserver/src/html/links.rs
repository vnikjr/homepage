use super::preprocessor::generic_preprocessor;
use maud::{Markup, html};

#[get("/links")]
pub fn links() -> Markup {
    generic_preprocessor(html! {
        main class = "container"{
            h2 { "reach out to me :3" }
            ul {
                li {
                    a href = "mailto:vlad@nikulin.name" { "email" }
                }
                li {
                    a { "discord username: vladiblo" }
                }
                li {
                    a href="github.com/vnikjr/" { "github" }
                }
            }
        }
    })
}
