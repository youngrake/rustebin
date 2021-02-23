use anyhow::Result;
use rocket::{http::Status, response::Response, Rocket};

use rocket_contrib::json::Json;

#[get("/<id>")]
fn fetch_user(id: String, conn: Db) -> Result<Js

pub fn fuel(rocket: Rocket) -> Rocket<Json> {
    todo!();
    rocket.mount("api/users", routes: R)
}
