use std::ops::DerefMut;

use rocket::http::{Cookies, Status};
use rocket::response::status::{self, Custom};
use rocket::Rocket;

use rocket_contrib::json::Json;
use serde_json::Value;
use users::repository::fetch_user;

use crate::core::pastes::entity::Paste;
use crate::core::pastes::repository::{create_paste, get_paste, update_paste};
use crate::core::users;

use crate::api::catchers::{forbidden, internal_error, not_found, unprocessable_entity};

use crate::utils::get_session_id;
use crate::utils::{db::DbConn, generate_id};

#[post("/", data = "<paste>")]
fn create(mut paste: Json<Paste>, conn: DbConn, mut cookies: Cookies) -> Custom<Json<Value>> {
    let user_id = get_session_id(&mut cookies);

    let user = match users::repository::find_or_create(user_id, &conn) {
        Ok(user) => user,
        Err(_) => return internal_error(),
    };

    let new_paste = paste.deref_mut();

    if new_paste.id.is_none() {
        new_paste.id = Some(generate_id());
    }

    new_paste.owner = Some(user.id);

    match create_paste(new_paste, &conn) {
        Ok(_) => status::Custom(
            Status::Created,
            Json(json!({
            "message": "Paste created",
            "id": new_paste.id
            })),
        ),

        Err(e) => status::Custom(
            Status::InternalServerError,
            Json(json!({
                "error": e.to_string(),
                "message": "Failed to create new paste"
            })),
        ),
    }
}

#[patch("/", data = "<paste>")]
fn update(mut paste: Json<Paste>, conn: DbConn, mut cookies: Cookies) -> Custom<Json<Value>> {
    let user_id = get_session_id(&mut cookies);

    let user = match fetch_user(user_id, &conn) {
        Ok(user) => user,
        Err(_) => return not_found(),
    };

    let upd_paste = paste.deref_mut();

    if upd_paste.id.is_none() {
        return not_found();
    }
    upd_paste.owner = match get_paste(upd_paste.id.as_ref().unwrap().clone(), &conn) {
        Ok(paste) => paste.owner,
        Err(_) => return internal_error(),
    };

    if upd_paste.owner.is_some() {
        if *upd_paste.owner.as_ref().unwrap() == user.id {
            match update_paste(upd_paste, &conn) {
                Ok(_) => status::Custom(
                    Status::Created,
                    Json(json!({
                        "message": "Successfully created paste",
                        "paste_id": upd_paste.id
                    })),
                ),
                Err(_) => internal_error(),
            }
        } else {
            forbidden()
        }
    } else {
        unprocessable_entity()
    }
}

#[get("/<id>")]
fn get(id: String, conn: DbConn, mut cookies: Cookies) -> Custom<Json<Value>> {
    let user_id = get_session_id(&mut cookies);

    let user = match fetch_user(user_id, &conn) {
        Ok(user) => user,
        Err(_) => return internal_error(),
    };

    let paste = match get_paste(id, &conn) {
        Ok(paste) => paste,
        Err(e) => return status::Custom(Status::NotFound, Json(json!({"error": e.to_string()}))),
    };

    let paste_owner = paste.owner.as_ref().unwrap();

    if user.id == *paste_owner {
        Custom(
            Status::Ok,
            Json(json!({
                "id": paste.id,
                "owner": paste_owner,
                "is_url": paste.is_url,
                "body": paste.body,
                "is_owner": true
            })),
        );
    }

    Custom(Status::Ok, Json(json!(paste)))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/paste", routes![create, get])
}
