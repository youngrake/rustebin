use rocket::response::status::Custom;
use rocket::{http::Status, Rocket};

use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/test")]
fn test() -> Custom<Json<Value>> {
    Custom(Status::Ok, Json(json!({"message": "Ok"})))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api", routes![test])
}
