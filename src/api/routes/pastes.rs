use std::ops::DerefMut;

use rocket::response::status::Custom;
use rocket::{
    http::{Cookie, Cookies, Status},
    response::status,
    Rocket,
};

use rocket_contrib::json::Json;
use serde_json::Value;

use crate::{
    core::pastes::{entity::Paste, orm::create_paste, repository::get_paste},
    utils::db::DbConn,
};

#[post("/", data = "<paste>")]
fn create(mut paste: Json<Paste>, conn: DbConn, mut cookies: Cookies) -> Custom<Json<Value>> {
    // let session = match cookies.get_private("session") {
    //     Some(c) => c.value().to_string(),
    //     None => {
    //         let user_id = "pepe";
    //         cookies.add_private(Cookie::new("session", user_id.to_string()));
    //         user_id.to_string()
    //     }
    // };

    let new_paste = paste.deref_mut();

    if new_paste.id.is_none() {
        new_paste.id = Some("pepe".to_string());
    }

    match create_paste(new_paste, &conn) {
        Ok(_) => status::Custom(
            Status::Created,
            Json(json!({
            "message": "Paste created",
            "id": new_paste.id}
                    )),
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

fn get(id: String, conn: DbConn, mut cookies: Cookies) -> Custom<Json<Value>> {
    todo!()
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api/paste", routes![create])
}
