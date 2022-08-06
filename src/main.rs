#[macro_use] extern crate rocket;

mod home_controller;

//noinspection RsMainFunctionNotFound
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        home_controller::index,
        home_controller::files
    ])
}