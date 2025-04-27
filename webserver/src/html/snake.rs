use super::preprocessor::generic_preprocessor;
use maud::{Markup, html};

#[get("/snake")]
pub fn snake() -> Markup {
    generic_preprocessor(html! {
        script type = "module"{"""
            import init, {greet} from '/wasm/snake.js';

            await init();

            greet('hiiiii')
        """}
    })
}
