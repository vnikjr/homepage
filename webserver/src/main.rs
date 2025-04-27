#[macro_use]
extern crate rocket;
use std::process::Command;

mod html;
mod wasm;

#[launch]
fn rocket() -> _ {
    Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--release", // when i uncomment this it says im using -r twice
            "../wasm/snake",
            "--out-dir",
            "../../webserver/static/wasm/snake",
            "--no-pack",
        ])
        .status()
        .expect("Failed to build wasm package");
    rocket::build()
        .mount("/wasm", wasm::get_routes())
        .mount("/", html::return_routes())
}
