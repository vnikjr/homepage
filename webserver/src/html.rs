use chrono::Datelike;
use maud::{DOCTYPE, Markup, html};
use rocket::Route;

pub fn return_routes() -> Vec<Route> {
    routes![index, projects, links, gayness]
}

fn generic_preprocessor(page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        (head())
        (header())
        (page)
        (footer())
        (konami_code()) // might add this at some point if i can get it to work
    }
}

fn footer() -> Markup {
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

fn head() -> Markup {
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
            link rel = "stylesheet" href = "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.purple.min.css"{}
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
fn konami_code() -> Markup {
    html! {
        div x-data="{ konami: [] }" "@keydown.window"="konami.push($event.key); if (konami.slice(-11).join('') === 'ArrowUpArrowUpArrowDownArrowDownArrowLeftArrowRightArrowLeftArrowRightbaEnter') { window.location.href = 'https://youtu.be/dQw4w9WgXcQ' }"{}
    }
}

#[get("/")]
fn index() -> Markup {
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

#[get("/projects")]
fn projects() -> Markup {
    generic_preprocessor(html! {
        head {
            style{ "

                  /* Base grid (fallback for non-Firefox) */
                  .projects-grid {
                    display: grid;
                    grid-template-columns: repeat(2, 1fr);
                    gap: 1rem;
                  }

                  /* Firefox-only masonry */
                  @supports (grid-template-rows: masonry) {
                    .projects-grid {
                      grid-template-rows: masonry;
                      align-items: start;
                    }
                    .project-tile {
                      height: auto !important;
                    }
                  }

                  /* Optional: Compact fallback for non-Firefox */
                  /* Uncomment if you prefer compact over stretchy */
                  /*
                  .projects-grid {
                    grid-auto-rows: min-content;
                    align-items: start;
                  }
                  .project-tile {
                    display: flex;
                    flex-direction: column;
                    height: 100%;
                  }
                  .project-tile footer {
                    margin-top: auto;
                  }
                  */
                  .project-tile {
                    display: flex;          /* Enable flexbox */
                    flex-direction: column; /* Stack children vertically */
                    height: 100%;           /* Fill entire grid cell */
                  }

                  .project-tile footer {
                    margin-top: auto; /* Pushes footer to bottom */
                    padding-top: 1rem; /* Optional spacing */
                  }
            "
            }
        }
        main class = "container"{
            section {
                h1{ "projects"}
                p{ "this is the stuff i made and actually remembered to write down :3" }
            }
            section {
                div class = "grid projects-grid"{
                    article id="article" class = "project-tile"{
                        header {
                            h4{ "this site (you are here)"}
                        }
                        p { "Timestamp: apr 2025" }
                        p { "its the site youre on"}
                        footer {
                            a href = "/projects/homepage" class = "secondary" {small{ "details" }}
                        }
                    }
                    article id="article" class = "project-tile"{
                        header {
                            h4{ "my Nixos dotfiles" }
                        }
                        p { "Timestamp: dec 2025" }
                        p { "a set of nixos dotfiles declaratively doccumenting everything in my home pc and laptop's configurations, from the installed packages, to the hyprland configs, its all there" }
                        footer {
                            a href = "/projects/nixos" class = "secondary" {small{ "details" }}
                            p { "" }
                            a href = "https://github.com/vnikjr/my_nix_dots" class = "secondary" {small{ "github" }}
                        }
                    }
                    article id="article" class = "project-tile" {
                        header {
                            h4{ "my homelab (running Nixos btw)" }
                        }
                        p { "Timestamp: apr 2025" }
                        p { "while not the most hardware impressive of things (its my old laptop), the software and software management is top notch, running Nixos for a purely declarative approach to everything, managing a cluster of both my own docker containers and ones from the web, including pihole for custom dns, caddy for reverse proxying and traffic routing, and hosting a personal docker registry for my own projects, all descrived in Nix, allowing for perfect reproducability, " }
                        p { "future plans for this homelab include,
                            - a home media server
                            - ci/cd pipeline for rebuilding docker containers on push
                            - ci/cd pipeline for updating the configs of the homelab on push
                            " }
                        footer {
                            a href = "/projects/homelab" class = "secondary" {small{ "details " }}
                            p { "" }
                            a href = "https://github.com/vnikjr/homelab_config" class = "secondary" {small{ "github" }}
                        }
                    }
                    article id="article" class = "project-tile"{
                        header {
                            h4{ "school website" }
                        }
                        p { "Timestamp: sep 2024" }
                        p { "uhhhh, yeah i kinda made a website for my school, while not the official one, it is endorsed by out school, and also is used by some of the studens, it hosts a digital version of the school's rulebook, and while ive not been updating it for some time now, its help up as is perfectly fine and still serves its main purpose" }
                        footer {
                            a href = "/projects/school_site" class = "secondary" {small{ "details " }}
                            p { "" }
                            a href = "https://wnik.me" class = "secondary" {small{ "website" }}
                        }
                    }
                }
            }
        }
    })
}

#[get("/links")]
fn links() -> Markup {
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

#[get("/imverygayformaddie")]
fn gayness() -> Markup {
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
