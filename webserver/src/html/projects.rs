use super::preprocessor::generic_preprocessor;
use maud::{Markup, html};

#[get("/projects")]
pub fn projects() -> Markup {
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
                        p { "future plans for this homelab include"}
                        li { " a home media server" }
                        li { " ci/cd pipeline for rebuilding docker containers on push" }
                        li { " ci/cd pipeline for updating the configs of the homelab on push" }

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
                        p { "uhhhh, yeah i kinda made a website for my school, while not the official one, it is endorsed by our school, and also is used by some of the students, it hosts a digital version of the school's rulebook, and while ive not been updating it for some time now, its held up as is perfectly fine and still serves its main purpose" }
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
