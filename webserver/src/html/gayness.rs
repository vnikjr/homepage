use super::preprocessor::{footer, head, konami_code};
use maud::{DOCTYPE, Markup, html};
#[get("/imverygayformaddie")]
pub fn gayness() -> Markup {
    html! {
        (DOCTYPE)
        (head())
        main class = "container"{
            div x-data = "{line: 1, line_amount: 10}" {
                div x-show = "line == 1" {
                    p { "Hi Maddie :3" }
                    p { "did you know how much i love you?" }
                }
                div x-show = "line == 2" {
                    p { "you probably did, but let me show you anyways :3" }
                }
                div x-show = "line == 3" {
                    p { "is my love for you... this big?" }
                    p style = "font-size: calc(100vh / 6)"{ "💜" }
                    p { "no... we must go bigger" }
                }
                div x-show = "line == 4" {
                    p { "is my love for you... this big?" }
                    p style = "font-size: calc(100vh / 4)"{ "💜" }
                    p { "were getting somewhere... but we must go bigger" }
                }
                div x-show = "line == 5" {
                    p { "is my love for you... this big?" }
                    p style = "font-size: calc(100vh / 2)"{ "💜" }
                    p { "ok thats better... but we must go bigger" }
                }
                div x-show = "line == 6" {
                    p { "is my love for you... this big?" }
                    p style = "font-size: calc(100vh / 1)"{ "💜" }
                    p { "ohh now were close... but we must go bigger" }
                }
                div x-show = "line == 7" {
                    p { "is my love for you... this big?" }
                    p style = "font-size: calc(100vh * 2)"{ "💜" }
                    p { "almost there... we must go bigger" }
                }
                div x-show = "line == 8" {
                    p { "is my love for you... this big?" }
                    p style = "font-size: calc(100vh * 4)"{ "💜" }
                    p { "Thats what im talking about!!!" }
                    p { "Welp, here we are, thats the end, thats your silly easter egg, there is nothing more" }
                }
                div x-show = "line == 8" {
                    p { "Seriously there is nothing more to there" }
                }
                div x-show = "line == 9" {
                    p { "im warning you, dont go further" }
                }
                div x-show = "line == 10" {
                    p { "you deserved this" }
                }
                button "@click" = "line++; if (line == 10) {window.location.href = 'https://youtu.be/dQw4w9WgXcQ'}; window.scrollTo({ top: 0})"{"continue"}
            }
        }
        (footer())
        (konami_code())
    }
}
