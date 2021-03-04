use std::ops::DerefMut;

use rocket::http::{Cookies, Status};
use rocket::response::status::{self, Custom};
use rocket::Rocket;

use rocket_contrib::json::Json;
use serde_json::Value;

use crate::core::pastes::entity::Paste;
use crate::core::pastes::repository::{create_paste, get_paste};
use crate::utils::{db::DbConn, generate_id};

#[post("/", data = "<paste>")]
fn create(mut paste: Json<Paste>, conn: DbConn, mut _cookies: Cookies) -> Custom<Json<Value>> {
    let new_paste = paste.deref_mut();

    if new_paste.id.is_none() {
        new_paste.id = Some(generate_id());
    }

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

#[get("/<id>")]
fn get(id: String, conn: DbConn, mut _cookies: Cookies) -> Custom<Json<Value>> {
    let paste = match get_paste(id, &conn) {
        Ok(paste) => paste,
        Err(e) => return status::Custom(Status::NotFound, Json(json!({"error": e.to_string()}))),
    };

    Custom(Status::Ok, Json(json!(paste)))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/paste", routes![create, get])
}
