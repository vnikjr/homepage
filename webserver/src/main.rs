#[macro_use]
extern crate rocket;

mod html;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", html::return_routes())
}
