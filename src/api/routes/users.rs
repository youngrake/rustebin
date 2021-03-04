use anyhow::Result;

use rocket::http::Status;
use rocket::response::{status::Custom, Response};
use rocket::Rocket;

use rocket_contrib::json::Json;
use serde_json::Value;

use crate::api::catchers::*;
use crate::core::users::{entity, repository};
use crate::utils::db::DbConn;

#[post("/", data = "<user>")]
fn create(mut user: Json<entity::User>, conn: DbConn) -> Custom<Json<Value>> {
    if user.username.is_none() || user.password.is_none() {
        return inprocessable_entity();
    }

    let user_id = user.id.clone();

    // let mut found_user = match

    not_found()
}

#[get("/<id>")]
fn get(id: String, conn: DbConn) -> Result<Json<entity::User>> {
    Ok(Json(repository::fetch_user(id, &conn)?))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/user", routes![get])
}
