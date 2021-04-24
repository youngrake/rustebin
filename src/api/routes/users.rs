use anyhow::Result;

use rocket::http::{Cookies, Status};
use rocket::response::{status, Response};
use rocket::Rocket;

use rocket_contrib::json::Json;
use serde_json::Value;

use crate::core::users::{entity, repository};
use crate::utils::db::DbConn;
use crate::{api::catchers::*, core::users::entity::User, utils::get_session_id};

#[post("/", data = "<user>")]
fn create(mut user: Json<entity::User>, conn: DbConn) -> status::Custom<Json<Value>> {
    if user.username.is_none() || user.password.is_none() {
        return unprocessable_entity();
    }

    let user_id = user.id.clone();

    let mut found_user = match repository::find_or_create(user_id, &conn) {
        Ok(user) => user,
        Err(_) => {
            return internal_error();
        }
    };

    found_user.activated = Some(true);

    let user = repository::update_user(&mut user, &conn);

    match user {
        Ok(u) => status::Custom(Status::Ok, Json(json!(u))),
        Err(_) => return internal_error(),
    }
}

#[get("/<id>")]
fn get(id: String, conn: DbConn) -> Result<Json<entity::User>> {
    Ok(Json(repository::fetch_user(id, &conn)?))
}

#[get("/get_me")]
fn get_me(conn: DbConn, mut cookies: Cookies) -> Result<Json<User>> {
    let id = get_session_id(&mut cookies);
    Ok(Json(repository::fetch_user(id, &conn)?))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/user", routes![get, create, get_me])
}
