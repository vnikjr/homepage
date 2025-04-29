use super::preprocessor::generic_preprocessor;
use maud::{Markup, html};

#[get("/snake")]
pub fn snake() -> Markup {
    generic_preprocessor(html! {
        main class="container" {
            section {
                canvas id = "snake_canvas" height = "1000" width = "1000"{

                }
                script type = "module"{"""
                    import init from '/wasm/snake.js';

                    await init();

                """}
            }
        }
    })
}
