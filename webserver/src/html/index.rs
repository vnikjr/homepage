use super::preprocessor::generic_preprocessor;
use maud::{Markup, html};

#[get("/")]
pub fn index() -> Markup {
    generic_preprocessor(html! {
        main class="container"{
            section {
                h1{
                    "Hi!, my name is eve, this is my personal home page :3"
                }
                p{
                    "i make stuff and sometimes remember what i made after some times :3"
                }
            }
            section {
                div class = "grid"{
                    a href="/projects" role = "button"{ "projects" }
                    a href="/links" role = "button"{ "contact me" }
                }
            }
            section {
                // reserved for possibly some kind of blog thing
            }
        }
    })
}
