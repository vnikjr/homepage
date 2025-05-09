use chrono::Datelike;
use maud::{DOCTYPE, Markup, html};

pub fn generic_preprocessor(page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        (head())
        (header())
        (page)
        (footer())
        (konami_code())
    }
}

pub fn footer() -> Markup {
    html! {
        footer class="container" {
            hr;
            p {
                "Made with ❤️ using "
                a href="https://rocket.rs" target="_blank" { "Rocket" } ", "
                a href="https://maud.lambda.xyz" target="_blank" { "Maud" } ", and "
                a href="https://picocss.com" target="_blank" { "Pico.css" } "."
            }
            p {
                a href="/about" { "About Me" } " • "
                a href="/projects" { "Projects" } " • "
                a href="/resume" { "Résumé" } " • "
                a href="/contact" { "Contact" }
            }
            p {
                (format!("© {} Eve. All rights reserved.", chrono::Utc::now().year()))
            }
        }
    }
}

pub fn head() -> Markup {
    html! {
        head{
            script defer="defer" src=r"https://cdn.jsdelivr.net/npm/@alpinejs/persist@3.x.x/dist/cdn.min.js" {""}
            script defer="defer" src=r"https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
            style { "

                  html, body {
                    height: 100%;
                    margin: 0;
                  }

                  body {
                    display: flex;
                    flex-direction: column;
                  }

                  main.container {
                    flex: 1;
                  }
               }
            "}
            link rel = "stylesheet" href = "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.violet.min.css"{}
            title {"Eve's homepage :3"}
        }
    }
}

fn header() -> Markup {
    html! {
        head {
            style{ "
                /*header styling*/
                header {
                  position: sticky;
                  top: 0;
                  background: var(--pico-form-element-background-color);
                  border-bottom: 1px solid var(--pico-primary);
                }
                .mainheader {
                    z-index: 999;
                }

                nav ul {
                  display: flex;
                  gap: 1rem;
                }

                nav li strong a {
                  color: var(--pico-primary); /* Accent color for site name */
                }

            "
            }
        }
        header class="container-fluid mainheader" {
            nav {
                ul style = "display: flex; gap: 1rem; align-items: center"{
                    li {
                        button x-init="document.documentElement.setAttribute('data-theme', darkmode ? 'dark' : 'light'); console.log('darkmode: ',darkmode)" x-data = "{darkmode: $persist(true)}" "@click" = "darkmode = !darkmode; document.documentElement.setAttribute('data-theme', darkmode ? 'dark' : 'light')" arialabel = "Toggle theme"{
                            span x-show = "darkmode"{"☀️"}
                            span x-show = "!darkmode"{"🌙"}
                        }
                    }
                    li { strong { a href="/" { "eve's homepage" } } }
                    li { a href="/projects" { "Projects" } }
                    li { a href="/links" { "Links" } }

                }
            }
        }
    }
}

#[allow(unused)]
pub fn konami_code() -> Markup {
    html! {
        div x-data="{ konami: [] }" "@keydown.window"="konami.push($event.key); if (konami.slice(-11).join('') === 'ArrowUpArrowUpArrowDownArrowDownArrowLeftArrowRightArrowLeftArrowRightbaEnter') { window.location.href = 'https://youtu.be/dQw4w9WgXcQ' }"{}
    }
}
