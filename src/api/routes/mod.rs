use rocket::Rocket;

pub mod pastes;
pub mod test;
pub mod users;

pub fn fuel(rocket: Rocket) -> Rocket {
    let mut rocket = rocket;

    let rocket = test::fuel(rocket);
    let rocket = pastes::fuel(rocket);

    rocket
}
