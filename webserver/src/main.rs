#[macro_use]
extern crate rocket;
use std::process::Command;

use rocket::fs::{FileServer, relative};

mod blog;
mod html;
mod wasm;

#[launch]
fn rocket() -> _ {
    Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--release",
            "./wasm/snake",
            "--out-dir",
            "../../webserver/static/wasm/snake",
            "--no-pack",
        ])
        .status()
        .expect("Failed to build wasm package");

    rocket::build()
        .mount("/wasm", wasm::get_routes())
        .mount("/", html::return_routes())
        .mount("/blog", blog::get_routes())
        .mount("/icons", FileServer::from("webserver/static/images/icons"))
}
