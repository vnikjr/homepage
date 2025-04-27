use std::process::Command;

fn main() {
    // Tell cargo to rerun if the WASM source changes
    // println!("cargo:rerun-if-changed=../wasm/snake/src");

    // Build the wasm target using wasm-pack
    Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "../wasm/snake",
            "--out-dir",
            "../../webserver/static/wasm/snake",
        ])
        .status()
        .expect("Failed to build wasm package");
}
