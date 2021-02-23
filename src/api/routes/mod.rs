use rocket::Rocket;

pub mod test;

pub fn fuel(rocket: Rocket) -> Rocket {
    let mut rocket = rocket;

    let rocket = test::fuel(rocket);

    rocket
}
