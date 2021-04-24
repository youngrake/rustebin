use rocket::{http::Status, response::status, Rocket};
use rocket_contrib::json::Json;
use serde_json::Value;

#[catch(400)]
pub fn bad_request() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::BadRequest,
        Json(json!({"error": "Bad Request", "message": "Server cannot process the request"})),
    )
}

#[catch(403)]
pub fn forbidden() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::Forbidden,
        Json(
            json!({"error": "Forbidden", "message": "You are not allowed to perform this request"}),
        ),
    )
}

#[catch(404)]
pub fn not_found() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::NotFound,
        Json(json!({"error": "Not Found", "message": "The resource does not exist"})),
    )
}

#[catch(422)]
pub fn unprocessable_entity() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::UnprocessableEntity,
        Json(
            json!({"error": "UnprocessableEntity", "message": "The given JSON cannot be processed"}),
        ),
    )
}

#[catch(500)]
pub fn internal_error() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::InternalServerError,
        Json(json!({"error": "InternalServerError", "message": "Something went wrong, try again"})),
    )
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.register(catchers![
        bad_request,
        forbidden,
        not_found,
        unprocessable_entity,
        internal_error
    ])
}
