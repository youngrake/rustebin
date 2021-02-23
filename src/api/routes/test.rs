use rocket::Rocket;

#[get("/test")]
fn test() -> &'static str {
    "Test"
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api", routes![test])
}
