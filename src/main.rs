#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod api;
// pub mod core;
// pub mod schema;

fn main() {
    let mut rocket = rocket::ignite();

    rocket = api::routes::fuel(rocket);

    rocket.launch();
}
