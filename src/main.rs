#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate slog;

use dotenv::dotenv;
use slog::Drain;
use slog_async;
use slog_term;

pub mod api;
pub mod core;
pub mod schema;
pub mod utils;

fn main() {
    dotenv().ok();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());

    let mut rocket = rocket::ignite();

    rocket = api::routes::fuel(rocket);

    rocket
        .manage(utils::db::init_pool())
        .manage(logger)
        .launch();
}
