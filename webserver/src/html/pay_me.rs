use super::preprocessor::generic_preprocessor;
use maud::{Markup, html};

#[get("/pay_me")]
pub fn pay_me() -> Markup {
    generic_preprocessor(html! {
        main class="container" {
            div hx-get = "/blog/embedable/pay_me" hx-trigger="load" hx-swap = "outerHTML"{}
        }
    })
}
